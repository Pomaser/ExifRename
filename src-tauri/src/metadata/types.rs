use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    Ok,
    Conflict,
    MissingMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub original_path: String,
    pub original_name: String,
    pub proposed_name: Option<String>,
    pub proposed_path: Option<String>,
    pub status: FileStatus,
    pub camera_model: Option<String>,
    pub datetime_str: Option<String>,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub entries: Vec<FileEntry>,
    pub total: usize,
    pub ok_count: usize,
    pub conflict_count: usize,
    pub missing_count: usize,
}

impl ScanResult {
    pub fn compute_counts(entries: &[FileEntry]) -> (usize, usize, usize) {
        let mut ok = 0;
        let mut conflict = 0;
        let mut missing = 0;
        for e in entries {
            match e.status {
                FileStatus::Ok => ok += 1,
                FileStatus::Conflict => conflict += 1,
                FileStatus::MissingMetadata => missing += 1,
            }
        }
        (ok, conflict, missing)
    }
}
