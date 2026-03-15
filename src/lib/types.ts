export type FileStatus = 'ok' | 'conflict' | 'missing_metadata';

export interface FileEntry {
  id: string;
  original_path: string;
  original_name: string;
  proposed_name: string | null;
  proposed_path: string | null;
  status: FileStatus;
  camera_model: string | null;
  datetime_str: string | null;
  selected: boolean;
}

export interface ScanResult {
  entries: FileEntry[];
  total: number;
  ok_count: number;
  conflict_count: number;
  missing_count: number;
}

export interface RenameRequest {
  selected_ids: string[];
}

export interface RenameResult {
  renamed: number;
  moved_to_noexif: number;
  skipped: number;
  log_path: string;
}

export interface RenameLogSummary {
  session_id: string;
  created_at: string;
  source_folder: string;
  log_path: string;
  operation_count: number;
}
