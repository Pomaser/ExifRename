<script lang="ts">
  import { renameLogs, appStatus, errorMessage, fileList, scanResult } from '../stores';
  import { getRenameLogs, undoRename } from '../tauri';
  import { onMount } from 'svelte';

  onMount(async () => {
    try { renameLogs.set(await getRenameLogs()); } catch {}
  });

  async function handleUndo(logPath: string) {
    appStatus.set('undoing');
    errorMessage.set('');
    try {
      await undoRename(logPath);
      renameLogs.set(await getRenameLogs());
      scanResult.set(null);
      fileList.set([]);
      appStatus.set('idle');
    } catch (e) {
      errorMessage.set(String(e));
      appStatus.set('ready');
    }
  }

  function formatDate(iso: string) {
    return new Date(iso).toLocaleString();
  }
</script>

{#if $renameLogs.length > 0}
  <div class="undo-panel">
    <div class="section-title">Rename history</div>
    {#each $renameLogs as log}
      <div class="log-entry">
        <div class="log-info">
          <span class="log-date">{formatDate(log.created_at)}</span>
          <span class="log-folder" title={log.source_folder}>{log.source_folder}</span>
          <span class="log-count">{log.operation_count} operations</span>
        </div>
        <button
          class="btn-undo"
          disabled={$appStatus === 'undoing'}
          on:click={() => handleUndo(log.log_path)}
        >
          Undo
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .undo-panel {
    border-top: 1px solid #2a2a2a;
    padding-top: 10px;
    flex-shrink: 0;
    max-height: 160px;
    overflow-y: auto;
  }
  .section-title {
    color: #555;
    font-size: 0.75em;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.07em;
    margin-bottom: 8px;
    position: sticky;
    top: 0;
    background: #1a1a1a;
    padding-top: 2px;
  }
  .log-entry {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 7px 10px;
    background: #222;
    border: 1px solid #2e2e2e;
    border-radius: 5px;
    margin-bottom: 5px;
    gap: 12px;
  }
  .log-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    overflow: hidden;
    min-width: 0;
  }
  .log-date   { color: #555; font-size: 0.78em; }
  .log-folder {
    font-family: monospace;
    font-size: 0.8em;
    color: #999;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .log-count { color: #444; font-size: 0.75em; }
  .btn-undo {
    background: #2e1f00;
    color: #e0931a;
    border: 1px solid #4a3500;
    border-radius: 5px;
    padding: 4px 12px;
    cursor: pointer;
    font-size: 0.82em;
    font-weight: 600;
    white-space: nowrap;
    transition: background 0.15s;
  }
  .btn-undo:hover:not(:disabled) { background: #3a2800; }
  .btn-undo:disabled { opacity: 0.35; cursor: not-allowed; }
</style>
