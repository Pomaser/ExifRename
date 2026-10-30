# ExifFileRenamer — kontext projektu

## Co to je
Desktopová aplikace (Rust + Tauri 2 + Svelte) pro přejmenování fotek a videí podle EXIF/QuickTime metadat.

## Stack
- **Backend:** Rust, Tauri 2, kamadak-exif (JPEG), vlastní QuickTime atom parser (MOV/MP4)
- **Frontend:** SvelteKit + adapter-static (SPA)
- **Build:** `NO_STRIP=1 APPIMAGE_EXTRACT_AND_RUN=1 npm run tauri build` (na Fedora/moderním Linuxu)
- **Dev:** `GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev`

## Klíčová rozhodnutí
- **Bez ffprobe** — videa se čtou nativně přes vlastní parser QuickTime atomů (`src-tauri/src/metadata/video_reader.rs`)
- **WebKit env vars** jsou nastaveny přímo v `main.rs` (pro správné vykreslení na Wayland/Fedora)
- `NO_STRIP=1` při buildu AppImage — linuxdeploy má starý `strip` nekompatibilní s Fedora 42 ELF formátem

## Struktura backendu
```
src-tauri/src/
  metadata/
    exif_reader.rs     — čtení JPEG EXIF (kamadak-exif)
    video_reader.rs    — nativní čtení MOV/MP4 (QuickTime atom parser)
    types.rs           — FileEntry, ScanResult, FileStatus
  commands/
    scan.rs            — walkdir + přečtení metadat + resolve_collisions
    rename.rs          — přejmenování + undo + JSON log
  renamer/mod.rs       — format_datetime, resolve_collisions
  log_manager/mod.rs   — JSON logy v ~/.local/share/ExifFileRenamer/logs/
  lib.rs               — Tauri commands, AppState
```

## Formát přejmenování
`YYYYMMDD_HHMMSS.ext` — kolize řeší suffixem `_1`, `_2`, ...
Kontrola kolize (`renamer::resolve_collisions` i `commands::rename::ensure_unique_path`) ignoruje případ, kdy navržená cesta je shodná s **vlastní** aktuální cestou souboru — jinak by opakovaný scan/rename už správně pojmenovaného souboru vyhodnotil self-match jako konflikt a zbytečně přidal `_1` (oprava v0.1.4).

## Priorita metadat u videí
1. `com.apple.quicktime.creationdate` (moov→meta→keys/ilst) — lokální čas s offsetem, použít přímo
2. `mvhd.creation_time` (Mac epoch 1904-01-01 → UTC → lokální čas systému)

## Podporované formáty
JPG, JPEG, MOV, MP4 — složka `NoExif/` pro soubory bez metadat

## Release
GitHub: https://github.com/Pomaser/ExifRename
Aktuální verze: v0.1.4
Release vznikne automaticky přes `.github/workflows/build.yml` při pushnutí tagu `v*` (build Linux + Windows, `tauri-apps/tauri-action`). Verzi je potřeba sjednotit ve `package.json`, `src-tauri/Cargo.toml` a `src-tauri/tauri.conf.json` před vytvořením tagu.
