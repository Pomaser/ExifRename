<script lang="ts">
  import { onMount } from 'svelte';
  import FolderSelector from '$lib/components/FolderSelector.svelte';
  import OptionsBar from '$lib/components/OptionsBar.svelte';
  import FileTable from '$lib/components/FileTable.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
  import UndoPanel from '$lib/components/UndoPanel.svelte';
  import {
    fileList,
    appStatus,
    scanResult,
    lastRenameResult,
    ffprobeAvailable,
    errorMessage,
  } from '$lib/stores';
  import { executeRename, checkFfprobe, getRenameLogs } from '$lib/tauri';
  import { renameLogs } from '$lib/stores';

  let showConfirm = false;

  onMount(async () => {
    ffprobeAvailable.set(await checkFfprobe());
    renameLogs.set(await getRenameLogs());
  });

  $: hasFiles = $fileList.length > 0;
  $: selectedCount = $fileList.filter(f => f.selected).length;
  $: isRenaming = $appStatus === 'renaming';

  function openConfirm() {
    showConfirm = true;
  }

  async function handleConfirm(e: CustomEvent<string[]>) {
    const selected_ids = e.detail;
    appStatus.set('renaming');
    errorMessage.set('');
    try {
      const result = await executeRename({ selected_ids });
      lastRenameResult.set({
        renamed: result.renamed,
        moved: result.moved_to_noexif,
        logPath: result.log_path,
      });
      renameLogs.set(await getRenameLogs());
      // Clear the list — user should rescan if they want to proceed
      scanResult.set(null);
      fileList.set([]);
      appStatus.set('done');
    } catch (e) {
      errorMessage.set(String(e));
      appStatus.set('ready');
    }
  }
</script>

<div class="app">
  <header>
    <h1>ExifFileRenamer</h1>
    {#if !$ffprobeAvailable}
      <div class="warning-banner">
        ffprobe not found — video files (MOV/MP4) will not be processed.
        Install FFmpeg and ensure it is on your PATH.
      </div>
    {/if}
  </header>

  <main>
    <section class="top-section">
      <FolderSelector />
      <OptionsBar />
    </section>

    {#if $errorMessage}
      <div class="error-banner">{$errorMessage}</div>
    {/if}

    {#if $appStatus === 'done' && $lastRenameResult}
      <div class="success-banner">
        Done! {$lastRenameResult.renamed} file{$lastRenameResult.renamed !== 1 ? 's' : ''} renamed,
        {$lastRenameResult.moved} moved to NoExif/.
      </div>
    {/if}

    {#if hasFiles}
      <div class="table-section">
        <div class="table-toolbar">
          <span class="selected-info">{selectedCount} of {$fileList.length} selected</span>
          <button
            class="btn-rename"
            disabled={selectedCount === 0 || isRenaming}
            on:click={openConfirm}
          >
            {isRenaming ? 'Renaming…' : 'Rename selected'}
          </button>
        </div>

        <FileTable />
        <StatusBar />
      </div>
    {:else if $appStatus === 'idle' || $appStatus === 'done'}
      <div class="empty-state">
        Select a folder and click "Scan folder" to begin.
      </div>
    {:else if $appStatus === 'scanning'}
      <div class="empty-state">Scanning…</div>
    {/if}

    <UndoPanel />
  </main>

  <ConfirmDialog bind:show={showConfirm} on:confirm={handleConfirm} />
</div>

<style>
  :global(*, *::before, *::after) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    background: #141414;
    color: #ddd;
    font-family: Inter, system-ui, sans-serif;
    font-size: 15px;
    line-height: 1.5;
    height: 100vh;
    overflow: hidden;
  }

  :global(input[type="checkbox"]) {
    accent-color: #4a9eff;
    width: 15px;
    height: 15px;
    cursor: pointer;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  header {
    padding: 14px 24px 0;
    flex-shrink: 0;
  }

  h1 {
    margin: 0 0 10px;
    font-size: 1.4em;
    font-weight: 700;
    color: #4a9eff;
    letter-spacing: -0.02em;
  }

  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 0 24px 16px;
    overflow: hidden;
    gap: 12px;
  }

  .top-section {
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .table-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    gap: 8px;
  }

  .table-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .selected-info {
    color: #888;
    font-size: 0.88em;
  }

  .btn-rename {
    background: #2e7a2e;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 8px 22px;
    font-size: 0.95em;
    font-weight: 600;
    cursor: pointer;
  }
  .btn-rename:hover:not(:disabled) { background: #3a9a3a; }
  .btn-rename:disabled { opacity: 0.4; cursor: not-allowed; }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #555;
    font-size: 1em;
  }

  .warning-banner {
    background: #3a2a00;
    color: #ff9800;
    border: 1px solid #ff9800;
    border-radius: 6px;
    padding: 8px 14px;
    font-size: 0.88em;
    margin-bottom: 4px;
  }

  .error-banner {
    background: #3a1a1a;
    color: #f44336;
    border: 1px solid #f44336;
    border-radius: 6px;
    padding: 8px 14px;
    font-size: 0.88em;
  }

  .success-banner {
    background: #1b3a1b;
    color: #4caf50;
    border: 1px solid #4caf50;
    border-radius: 6px;
    padding: 8px 14px;
    font-size: 0.88em;
  }
</style>
