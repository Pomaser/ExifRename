use crate::log_manager::{write_log, RenameOperation};
use crate::metadata::types::{FileStatus, ScanResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct RenameRequest {
    pub selected_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RenameResult {
    pub renamed: usize,
    pub moved_to_noexif: usize,
    pub skipped: usize,
    pub log_path: String,
}

pub fn execute_rename_impl(
    scan_result: &ScanResult,
    request: &RenameRequest,
    source_folder: &str,
) -> Result<RenameResult, String> {
    let selected_set: std::collections::HashSet<&String> = request.selected_ids.iter().collect();

    let mut operations: Vec<RenameOperation> = Vec::new();
    let mut renamed = 0usize;
    let mut moved_to_noexif = 0usize;
    let mut skipped = 0usize;

    for entry in &scan_result.entries {
        if !selected_set.contains(&entry.id) {
            skipped += 1;
            continue;
        }

        match entry.status {
            FileStatus::MissingMetadata => {
                // Move to NoExif subfolder
                let noexif_dir = Path::new(source_folder).join("NoExif");
                fs::create_dir_all(&noexif_dir).map_err(|e| e.to_string())?;

                let dest = ensure_unique_path(noexif_dir.join(&entry.original_name));
                fs::rename(&entry.original_path, &dest).map_err(|e| {
                    format!(
                        "Failed to move {} to NoExif: {}",
                        entry.original_name, e
                    )
                })?;

                operations.push(RenameOperation {
                    from_path: entry.original_path.clone(),
                    to_path: dest.to_string_lossy().to_string(),
                    moved_to_noexif: true,
                });
                moved_to_noexif += 1;
            }
            FileStatus::Ok | FileStatus::Conflict => {
                let proposed_path = match &entry.proposed_path {
                    Some(p) => p.clone(),
                    None => {
                        skipped += 1;
                        continue;
                    }
                };

                // Final filesystem collision check at rename time
                let final_path = ensure_unique_path(std::path::PathBuf::from(&proposed_path));

                fs::rename(&entry.original_path, &final_path).map_err(|e| {
                    format!("Failed to rename {}: {}", entry.original_name, e)
                })?;

                operations.push(RenameOperation {
                    from_path: entry.original_path.clone(),
                    to_path: final_path.to_string_lossy().to_string(),
                    moved_to_noexif: false,
                });
                renamed += 1;
            }
        }
    }

    let log_path = write_log(source_folder, operations)?;

    Ok(RenameResult {
        renamed,
        moved_to_noexif,
        skipped,
        log_path,
    })
}

fn ensure_unique_path(path: std::path::PathBuf) -> std::path::PathBuf {
    if !path.exists() {
        return path;
    }
    let stem = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let parent = path.parent().unwrap_or(Path::new("."));

    let mut counter = 1usize;
    loop {
        let candidate = parent.join(format!("{}_{}{}", stem, counter, ext));
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

pub fn undo_rename_impl(log_path: &str) -> Result<usize, String> {
    let log = crate::log_manager::read_log(log_path)?;
    let mut count = 0usize;

    // Reverse in reverse order
    for op in log.operations.iter().rev() {
        if let Err(e) = fs::rename(&op.to_path, &op.from_path) {
            return Err(format!(
                "Failed to undo rename {} → {}: {}",
                op.to_path, op.from_path, e
            ));
        }
        count += 1;
    }

    // Clean up empty NoExif directory if applicable
    if let Some(parent) = Path::new(&log.source_folder).to_path_buf().parent() {
        let noexif = parent.join("NoExif");
        if noexif.is_dir() {
            fs::remove_dir(&noexif).ok(); // only removes if empty
        }
    }
    let noexif = Path::new(&log.source_folder).join("NoExif");
    fs::remove_dir(&noexif).ok();

    // Remove log file after successful undo
    fs::remove_file(log_path).ok();

    Ok(count)
}
