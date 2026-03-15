use crate::metadata::{
    exif_reader::read_image_metadata,
    video_reader::read_video_datetime,
    types::{FileEntry, FileStatus, ScanResult},
};
use crate::renamer;
use std::path::Path;
use uuid::Uuid;
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "mov", "mp4"];

pub fn scan_folder_impl(folder: &str, recursive: bool) -> ScanResult {
    let folder_path = Path::new(folder);
    let mut entries: Vec<FileEntry> = Vec::new();
    let mut proposals: Vec<(String, String, String)> = Vec::new(); // (id, stem, ext)

    let walker = if recursive {
        WalkDir::new(folder_path).max_depth(usize::MAX)
    } else {
        WalkDir::new(folder_path).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Skip the NoExif subdirectory
        if path
            .components()
            .any(|c| c.as_os_str() == "NoExif")
        {
            continue;
        }

        if !path.is_file() {
            continue;
        }

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => continue,
        };

        if !SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }

        let id = Uuid::new_v4().to_string();
        let original_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let original_path = path.to_string_lossy().to_string();

        let datetime_opt = match ext.as_str() {
            "jpg" | "jpeg" => {
                read_image_metadata(path).map(|m| (m.datetime, m.camera_model))
            }
            "mov" | "mp4" => read_video_datetime(path).map(|dt| (dt, None)),
            _ => None,
        };

        if let Some((dt, camera_model)) = datetime_opt {
            let stem = renamer::format_datetime(&dt);
            let datetime_str = dt.format("%Y-%m-%d %H:%M:%S").to_string();
            proposals.push((id.clone(), stem.clone(), ext.clone()));

            entries.push(FileEntry {
                id,
                original_path,
                original_name,
                proposed_name: None, // filled after collision resolution
                proposed_path: None,
                status: FileStatus::Ok,
                camera_model,
                datetime_str: Some(datetime_str),
                selected: true,
            });
        } else {
            entries.push(FileEntry {
                id,
                original_path,
                original_name,
                proposed_name: None,
                proposed_path: None,
                status: FileStatus::MissingMetadata,
                camera_model: None,
                datetime_str: None,
                selected: true,
            });
        }
    }

    // Resolve collisions for entries that have metadata
    let collision_map = renamer::resolve_collisions(&proposals, folder_path);

    // Fill in proposed names and update statuses
    for entry in &mut entries {
        if entry.status == FileStatus::MissingMetadata {
            continue;
        }
        if let Some((proposed_name, is_conflict)) = collision_map.get(&entry.id) {
            let proposed_path = folder_path
                .join(proposed_name)
                .to_string_lossy()
                .to_string();
            entry.proposed_name = Some(proposed_name.clone());
            entry.proposed_path = Some(proposed_path);
            if *is_conflict {
                entry.status = FileStatus::Conflict;
            }
        }
    }

    let total = entries.len();
    let (ok_count, conflict_count, missing_count) = ScanResult::compute_counts(&entries);

    ScanResult {
        entries,
        total,
        ok_count,
        conflict_count,
        missing_count,
    }
}
