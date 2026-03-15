<script lang="ts">
  import { recursive, folderPath, appStatus, fileList, scanResult, errorMessage } from '../stores';
  import { scanFolder } from '../tauri';

  async function handleScan() {
    if (!$folderPath) return;
    appStatus.set('scanning');
    errorMessage.set('');
    try {
      const result = await scanFolder($folderPath, $recursive);
      scanResult.set(result);
      fileList.set(result.entries);
      appStatus.set('ready');
    } catch (e) {
      errorMessage.set(String(e));
      appStatus.set('idle');
    }
  }
</script>

<div class="options-bar">
  <label class="checkbox-label">
    <input type="checkbox" bind:checked={$recursive} />
    Recursive (include subfolders)
  </label>
  <button
    class="btn-scan"
    disabled={!$folderPath || $appStatus === 'scanning'}
    on:click={handleScan}
  >
    {$appStatus === 'scanning' ? 'Scanning…' : 'Scan folder'}
  </button>
</div>

<style>
  .options-bar {
    display: flex;
    align-items: center;
    gap: 16px;
    flex-shrink: 0;
  }
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 6px;
    color: #888;
    cursor: pointer;
    user-select: none;
    font-size: 0.88em;
  }
  .btn-scan {
    background: #1a3a5c;
    color: #6ab4ff;
    border: 1px solid #2a5a8c;
    border-radius: 5px;
    padding: 6px 18px;
    cursor: pointer;
    font-size: 0.88em;
    font-weight: 600;
    transition: background 0.15s;
  }
  .btn-scan:hover:not(:disabled) { background: #1f4570; }
  .btn-scan:disabled { opacity: 0.35; cursor: not-allowed; }
</style>
