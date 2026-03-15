import { writable } from 'svelte/store';
import type { FileEntry, ScanResult, RenameLogSummary } from './types';

export type AppStatus =
  | 'idle'
  | 'scanning'
  | 'ready'
  | 'renaming'
  | 'done'
  | 'undoing';

export const folderPath = writable<string>('');
export const recursive = writable<boolean>(false);
export const appStatus = writable<AppStatus>('idle');
export const scanResult = writable<ScanResult | null>(null);
export const fileList = writable<FileEntry[]>([]);
export const lastRenameResult = writable<{ renamed: number; moved: number; logPath: string } | null>(null);
export const renameLogs = writable<RenameLogSummary[]>([]);
export const ffprobeAvailable = writable<boolean>(true);
export const errorMessage = writable<string>('');
