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
      const file = item.getAsFile();
      if (file) {
        // @ts-ignore - Tauri exposes path on File
        const p: string = file.path ?? '';
        if (p) { folderPath.set(p); return; }
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
    <span class="folder-path">{$folderPath}</span>
    <button class="btn" on:click={pickFolder}>Change folder</button>
  {:else}
    <span class="hint">Drop a folder here or</span>
    <button class="btn btn-primary" on:click={pickFolder}>Browse…</button>
  {/if}
</div>

<style>
  .drop-zone {
    display: flex;
    align-items: center;
    gap: 12px;
    border: 1px dashed #383838;
    border-radius: 6px;
    padding: 12px 16px;
    background: #222;
    transition: border-color 0.15s, background 0.15s;
    flex-shrink: 0;
  }
  .drop-zone.dragging {
    border-color: #4a9eff;
    background: #1a2535;
  }
  .hint {
    color: #555;
    font-size: 0.88em;
  }
  .folder-path {
    flex: 1;
    font-family: monospace;
    font-size: 0.85em;
    color: #b0b0b0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .btn {
    background: #2a2a2a;
    color: #b0b0b0;
    border: 1px solid #3a3a3a;
    border-radius: 5px;
    padding: 5px 14px;
    cursor: pointer;
    font-size: 0.85em;
    white-space: nowrap;
    transition: background 0.15s;
  }
  .btn:hover { background: #333; }
  .btn-primary {
    background: #1a3a5c;
    color: #6ab4ff;
    border-color: #2a5a8c;
  }
  .btn-primary:hover { background: #1f4570; }
</style>
