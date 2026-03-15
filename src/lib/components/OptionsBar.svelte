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
    gap: 20px;
    padding: 10px 0;
  }
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 6px;
    color: #ccc;
    cursor: pointer;
    user-select: none;
  }
  .btn-scan {
    background: #4a9eff;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 8px 22px;
    cursor: pointer;
    font-size: 0.95em;
    font-weight: 600;
  }
  .btn-scan:hover:not(:disabled) { background: #3a8eef; }
  .btn-scan:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
