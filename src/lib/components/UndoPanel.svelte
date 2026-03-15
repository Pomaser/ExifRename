<script lang="ts">
  import { renameLogs, appStatus, errorMessage, fileList, scanResult } from '../stores';
  import { getRenameLogs, undoRename } from '../tauri';
  import { onMount } from 'svelte';

  onMount(async () => {
    try {
      renameLogs.set(await getRenameLogs());
    } catch {}
  });

  async function handleUndo(logPath: string) {
    appStatus.set('undoing');
    errorMessage.set('');
    try {
      await undoRename(logPath);
      renameLogs.set(await getRenameLogs());
      // Clear the scan result so user rescans
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
    <h3>Rename history</h3>
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
    border-top: 1px solid #333;
    padding-top: 16px;
    margin-top: 8px;
  }
  h3 { color: #aaa; font-size: 0.9em; margin: 0 0 10px; text-transform: uppercase; letter-spacing: 0.05em; }
  .log-entry {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px;
    background: #1e1e1e;
    border-radius: 6px;
    margin-bottom: 6px;
    gap: 12px;
  }
  .log-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
  }
  .log-date { color: #888; font-size: 0.8em; }
  .log-folder {
    font-family: monospace;
    font-size: 0.82em;
    color: #ccc;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .log-count { color: #666; font-size: 0.78em; }
  .btn-undo {
    background: #3a2a00;
    color: #ff9800;
    border: 1px solid #ff9800;
    border-radius: 6px;
    padding: 5px 14px;
    cursor: pointer;
    font-size: 0.85em;
    white-space: nowrap;
  }
  .btn-undo:hover:not(:disabled) { background: #4a3800; }
  .btn-undo:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
