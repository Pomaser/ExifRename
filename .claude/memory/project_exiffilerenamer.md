---
name: ExifFileRenamer project context
description: Tech stack, architecture, and current state of ExifFileRenamer Tauri 2 app
type: project
---

Rust + Tauri 2 + Svelte desktop app for renaming photos/videos based on EXIF metadata.

**Stack:**
- Backend: Rust, Tauri 2, kamadak-exif, ffprobe (subprocess), walkdir, chrono, uuid
- Frontend: SvelteKit + TypeScript (in src/routes/)
- Dialog: tauri-plugin-dialog v2 with `blocking` feature

**Features:**
- Folder picker (dialog + drag & drop)
- Optional recursive scan
- Supported: JPG, JPEG, MOV, MP4
- Rename format: YYYYMMDD_HHMMSS.EXT
- Preview table with OK/Conflict/NoMetadata status
- Files without metadata → moved to NoExif/ subfolder
- JSON rename log stored in $HOME/.local/share/ExifFileRenamer/logs/ for undo

**Module structure (src-tauri/src/):**
- `metadata/exif_reader.rs` — kamadak-exif for JPG/JPEG
- `metadata/ffprobe_reader.rs` — ffprobe subprocess for MOV/MP4
- `metadata/types.rs` — FileEntry, ScanResult, FileStatus
- `commands/scan.rs` — scan_folder_impl
- `commands/rename.rs` — execute_rename_impl, undo_rename_impl
- `renamer/mod.rs` — format_datetime, resolve_collisions
- `log_manager/mod.rs` — write/read/list JSON rename logs
- `lib.rs` — Tauri commands, AppState (scan_cache + scan_folder Mutexes)

**Frontend (src/):**
- `lib/types.ts` — TypeScript mirrors of Rust structs
- `lib/tauri.ts` — typed invoke() wrappers
- `lib/stores.ts` — Svelte stores
- `lib/components/` — FolderSelector, OptionsBar, FileTable, StatusBar, ConfirmDialog, UndoPanel
- `routes/+page.svelte` — main page wiring all components

**System prerequisites (Linux/Fedora):**
```
sudo dnf install -y webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel
```
ffprobe (ffmpeg) required on PATH for video files.

**Current state:** All code written, needs system packages to compile and test.

**Why:** User wanted a GUI app that replicates functionality of the bash script exifrename.sh.
**How to apply:** When resuming work, install system deps first, then `npm run tauri dev` to test.
