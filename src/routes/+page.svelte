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
    renameLogs,
  } from '$lib/stores';
  import { executeRename, checkFfprobe, getRenameLogs } from '$lib/tauri';

  let showConfirm = false;

  onMount(async () => {
    try {
      ffprobeAvailable.set(await checkFfprobe());
    } catch {
      ffprobeAvailable.set(false);
    }
    try {
      renameLogs.set(await getRenameLogs());
    } catch {
      renameLogs.set([]);
    }
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
      <div class="banner banner-warn">
        ffprobe not found — video files (MOV/MP4) will not be processed.
      </div>
    {/if}
  </header>

  <main>
    <FolderSelector />
    <OptionsBar />

    {#if $errorMessage}
      <div class="banner banner-error">{$errorMessage}</div>
    {/if}

    {#if $appStatus === 'done' && $lastRenameResult}
      <div class="banner banner-ok">
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
      <div class="empty-state">Select a folder and click "Scan folder" to begin.</div>
    {:else if $appStatus === 'scanning'}
      <div class="empty-state">Scanning…</div>
    {/if}

    <UndoPanel />
  </main>
</div>

<ConfirmDialog bind:show={showConfirm} on:confirm={handleConfirm} />

<style>
  :global(*, *::before, *::after) { box-sizing: border-box; }

  :global(html, body) {
    height: 100%;
    margin: 0;
    padding: 0;
    background: #1a1a1a;
    color: #e0e0e0;
    font-family: Inter, system-ui, sans-serif;
    font-size: 14px;
    line-height: 1.5;
    overflow: hidden;
  }

  :global(input[type="checkbox"]) {
    accent-color: #4a9eff;
    width: 14px;
    height: 14px;
    cursor: pointer;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #1a1a1a;
  }

  header {
    flex-shrink: 0;
    padding: 14px 20px 10px;
    border-bottom: 1px solid #2e2e2e;
    background: #1a1a1a;
  }

  h1 {
    margin: 0 0 8px;
    font-size: 1.3em;
    font-weight: 700;
    color: #4a9eff;
    letter-spacing: -0.02em;
  }

  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 12px 20px 14px;
    overflow: hidden;
    gap: 10px;
    background: #1a1a1a;
    min-height: 0;
  }

  .table-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    gap: 6px;
    min-height: 0;
  }

  .table-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .selected-info {
    color: #777;
    font-size: 0.85em;
  }

  .btn-rename {
    background: #1e5c1e;
    color: #7eda7e;
    border: 1px solid #2e7a2e;
    border-radius: 5px;
    padding: 6px 18px;
    font-size: 0.88em;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-rename:hover:not(:disabled) { background: #256025; }
  .btn-rename:disabled { opacity: 0.35; cursor: not-allowed; }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #444;
    font-size: 0.9em;
  }

  .banner {
    border-radius: 5px;
    padding: 7px 12px;
    font-size: 0.85em;
    flex-shrink: 0;
  }
  .banner-warn  { background: #2a1f00; color: #e6a817; border: 1px solid #5a3f00; }
  .banner-error { background: #2a1010; color: #e05050; border: 1px solid #5a2020; }
  .banner-ok    { background: #0f2a0f; color: #5ecf5e; border: 1px solid #1e5a1e; }
</style>
