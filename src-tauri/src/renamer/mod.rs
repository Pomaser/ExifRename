use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Format datetime to YYYYMMDD_HHMMSS
pub fn format_datetime(dt: &NaiveDateTime) -> String {
    dt.format("%Y%m%d_%H%M%S").to_string()
}

/// Given a list of (id, proposed_stem), detect intra-batch conflicts.
/// Returns a map from id -> (final_proposed_name, has_conflict).
pub fn resolve_collisions(
    proposals: &[(String, String, String)], // (id, stem, ext)
    folder: &Path,
) -> HashMap<String, (String, bool)> {
    // Count occurrences of each stem to detect batch conflicts
    let mut stem_count: HashMap<String, usize> = HashMap::new();
    for (_, stem, _) in proposals {
        *stem_count.entry(stem.clone()).or_insert(0) += 1;
    }

    // For each group with conflicts, assign suffixes
    let mut stem_index: HashMap<String, usize> = HashMap::new();
    let mut result = HashMap::new();

    for (id, stem, ext) in proposals {
        let count = stem_count[stem.as_str()];
        let has_batch_conflict = count > 1;

        let proposed_name = if has_batch_conflict {
            let idx = stem_index.entry(stem.clone()).or_insert(1);
            let name = format!("{}_{}.{}", stem, idx, ext.to_uppercase());
            *idx += 1;
            name
        } else {
            format!("{}.{}", stem, ext.to_uppercase())
        };

        // Also check filesystem for pre-existing files
        let final_name = ensure_no_fs_conflict(folder, &proposed_name);
        let is_conflict = has_batch_conflict || final_name != proposed_name;

        result.insert(id.clone(), (final_name, is_conflict));
    }

    result
}

/// If a file already exists on disk (and is not in the current batch), find a free name
fn ensure_no_fs_conflict(folder: &Path, proposed: &str) -> String {
    let mut candidate = PathBuf::from(folder).join(proposed);
    if !candidate.exists() {
        return proposed.to_string();
    }

    // Split stem and extension
    let path = Path::new(proposed);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path.extension().unwrap_or_default().to_string_lossy();

    let mut counter = 1usize;
    loop {
        let name = if ext.is_empty() {
            format!("{}_{}", stem, counter)
        } else {
            format!("{}_{}.{}", stem, counter, ext)
        };
        candidate = PathBuf::from(folder).join(&name);
        if !candidate.exists() {
            return name;
        }
        counter += 1;
    }
}
