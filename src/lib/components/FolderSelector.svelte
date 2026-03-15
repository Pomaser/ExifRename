<script lang="ts">
  import { openFolderDialog } from '../tauri';
  import { folderPath } from '../stores';

  let dragging = false;

  async function pickFolder() {
    const path = await openFolderDialog();
    if (path) folderPath.set(path);
  }

  function onDragOver(e: DragEvent) {
    e.preventDefault();
    dragging = true;
  }

  function onDragLeave() {
    dragging = false;
  }

  function onDrop(e: DragEvent) {
    e.preventDefault();
    dragging = false;
    const items = e.dataTransfer?.items;
    if (!items) return;
    for (const item of Array.from(items)) {
      const entry = item.webkitGetAsEntry?.();
      if (entry?.isDirectory) {
        // Tauri provides full path via the File object
        const file = item.getAsFile();
        if (file) {
          // @ts-ignore - Tauri exposes path on File
          const p: string = file.path ?? '';
          if (p) {
            folderPath.set(p);
            return;
          }
        }
      }
    }
  }
</script>

<div
  class="drop-zone"
  class:dragging
  on:dragover={onDragOver}
  on:dragleave={onDragLeave}
  on:drop={onDrop}
  role="region"
  aria-label="Drop zone for folder"
>
  {#if $folderPath}
    <p class="folder-path">{$folderPath}</p>
    <button class="btn-secondary" on:click={pickFolder}>Change folder</button>
  {:else}
    <p class="hint">Drop a folder here or</p>
    <button class="btn-primary" on:click={pickFolder}>Browse…</button>
  {/if}
</div>

<style>
  .drop-zone {
    border: 2px dashed #555;
    border-radius: 8px;
    padding: 24px 32px;
    text-align: center;
    transition: border-color 0.2s, background 0.2s;
    background: #1a1a1a;
  }
  .drop-zone.dragging {
    border-color: #4a9eff;
    background: #1a2a3a;
  }
  .hint {
    color: #888;
    margin: 0 0 12px;
  }
  .folder-path {
    font-family: monospace;
    font-size: 0.9em;
    color: #ccc;
    word-break: break-all;
    margin: 0 0 12px;
  }
  .btn-primary {
    background: #4a9eff;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 8px 20px;
    cursor: pointer;
    font-size: 0.95em;
  }
  .btn-primary:hover { background: #3a8eef; }
  .btn-secondary {
    background: #333;
    color: #ccc;
    border: 1px solid #555;
    border-radius: 6px;
    padding: 6px 16px;
    cursor: pointer;
    font-size: 0.9em;
  }
  .btn-secondary:hover { background: #444; }
</style>
