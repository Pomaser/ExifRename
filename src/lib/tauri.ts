import { invoke } from '@tauri-apps/api/core';
import type { ScanResult, RenameRequest, RenameResult, RenameLogSummary } from './types';

export async function openFolderDialog(): Promise<string | null> {
  return invoke<string | null>('open_folder_dialog');
}

export async function scanFolder(folder: string, recursive: boolean): Promise<ScanResult> {
  return invoke<ScanResult>('scan_folder', { folder, recursive });
}

export async function executeRename(request: RenameRequest): Promise<RenameResult> {
  return invoke<RenameResult>('execute_rename', { request });
}

export async function undoRename(logPath: string): Promise<number> {
  return invoke<number>('undo_rename', { logPath });
}

export async function getRenameLogs(): Promise<RenameLogSummary[]> {
  return invoke<RenameLogSummary[]>('get_rename_logs');
}

