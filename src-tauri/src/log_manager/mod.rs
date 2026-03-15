use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameOperation {
    pub from_path: String,
    pub to_path: String,
    pub moved_to_noexif: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameLog {
    pub session_id: String,
    pub created_at: String,
    pub source_folder: String,
    pub operations: Vec<RenameOperation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenameLogSummary {
    pub session_id: String,
    pub created_at: String,
    pub source_folder: String,
    pub log_path: String,
    pub operation_count: usize,
}

pub fn logs_directory() -> PathBuf {
    let base = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ExifFileRenamer")
        .join("logs");
    fs::create_dir_all(&base).ok();
    base
}

pub fn write_log(source_folder: &str, operations: Vec<RenameOperation>) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    let log = RenameLog {
        session_id: session_id.clone(),
        created_at: Utc::now().to_rfc3339(),
        source_folder: source_folder.to_string(),
        operations,
    };

    let log_path = logs_directory().join(format!("{}.json", session_id));
    let json = serde_json::to_string_pretty(&log).map_err(|e| e.to_string())?;
    fs::write(&log_path, json).map_err(|e| e.to_string())?;
    Ok(log_path.to_string_lossy().to_string())
}

pub fn read_log(log_path: &str) -> Result<RenameLog, String> {
    let content = fs::read_to_string(log_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

pub fn list_logs() -> Vec<RenameLogSummary> {
    let dir = logs_directory();
    let mut summaries = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(log) = serde_json::from_str::<RenameLog>(&content) {
                    summaries.push(RenameLogSummary {
                        session_id: log.session_id,
                        created_at: log.created_at,
                        source_folder: log.source_folder,
                        log_path: path.to_string_lossy().to_string(),
                        operation_count: log.operations.len(),
                    });
                }
            }
        }
    }

    // Sort by created_at descending
    summaries.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    summaries
}
