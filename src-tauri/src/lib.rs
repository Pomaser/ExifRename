mod commands;
mod log_manager;
mod metadata;
mod renamer;

use commands::rename::{execute_rename_impl, undo_rename_impl, RenameRequest, RenameResult};
use commands::scan::scan_folder_impl;
use log_manager::{list_logs, RenameLogSummary};
use metadata::types::ScanResult;
use std::sync::Mutex;

struct AppState {
    scan_cache: Mutex<Option<ScanResult>>,
    scan_folder: Mutex<Option<String>>,
}

#[tauri::command]
async fn open_folder_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    use tokio::sync::oneshot;

    let (tx, rx) = oneshot::channel();
    app.dialog().file().pick_folder(move |path| {
        let _ = tx.send(path);
    });
    let path = rx.await.map_err(|e| e.to_string())?;
    Ok(path.map(|p| p.to_string()))
}

#[tauri::command]
async fn scan_folder(
    folder: String,
    recursive: bool,
    state: tauri::State<'_, AppState>,
) -> Result<ScanResult, String> {
    let result = scan_folder_impl(&folder, recursive);
    *state.scan_cache.lock().unwrap() = Some(result.clone());
    *state.scan_folder.lock().unwrap() = Some(folder);
    Ok(result)
}

#[tauri::command]
async fn execute_rename(
    request: RenameRequest,
    state: tauri::State<'_, AppState>,
) -> Result<RenameResult, String> {
    let scan_result = state
        .scan_cache
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| "No scan result available. Please scan a folder first.".to_string())?;

    let folder = state
        .scan_folder
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| "No source folder available.".to_string())?;

    execute_rename_impl(&scan_result, &request, &folder)
}

#[tauri::command]
async fn undo_rename(log_path: String) -> Result<usize, String> {
    undo_rename_impl(&log_path)
}

#[tauri::command]
async fn get_rename_logs() -> Result<Vec<RenameLogSummary>, String> {
    Ok(list_logs())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            scan_cache: Mutex::new(None),
            scan_folder: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            open_folder_dialog,
            scan_folder,
            execute_rename,
            undo_rename,
            get_rename_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
