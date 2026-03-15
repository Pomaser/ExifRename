<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { fileList } from '../stores';

  export let show = false;

  const dispatch = createEventDispatcher<{ confirm: string[]; cancel: void }>();

  $: selectedEntries = $fileList.filter(f => f.selected);
  $: toRename = selectedEntries.filter(f => f.status === 'ok' || f.status === 'conflict').length;
  $: toNoExif = selectedEntries.filter(f => f.status === 'missing_metadata').length;

  function confirm() {
    const ids = selectedEntries.map(f => f.id);
    dispatch('confirm', ids);
    show = false;
  }

  function cancel() {
    dispatch('cancel');
    show = false;
  }
</script>

{#if show}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="overlay" on:click={cancel} role="button" tabindex="-1">
    <div class="dialog" on:click|stopPropagation role="dialog" aria-modal="true" tabindex="-1">
      <h2>Confirm renaming</h2>

      <div class="summary">
        <p>
          <span class="num ok">{toRename}</span>
          file{toRename !== 1 ? 's' : ''} will be renamed
        </p>
        {#if toNoExif > 0}
          <p>
            <span class="num missing">{toNoExif}</span>
            file{toNoExif !== 1 ? 's' : ''} will be moved to <code>NoExif/</code>
          </p>
        {/if}
      </div>

      <p class="warning">This action cannot be undone without using the Undo feature.</p>

      <div class="actions">
        <button class="btn-cancel" on:click={cancel}>Cancel</button>
        <button class="btn-confirm" on:click={confirm} disabled={selectedEntries.length === 0}>
          Rename
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }
  .dialog {
    background: #252525;
    border: 1px solid #444;
    border-radius: 10px;
    padding: 28px 32px;
    min-width: 380px;
    max-width: 500px;
  }
  h2 { margin: 0 0 20px; color: #eee; font-size: 1.2em; }
  .summary { margin-bottom: 16px; }
  .summary p { margin: 6px 0; color: #ccc; }
  .num { font-weight: 700; font-size: 1.1em; margin-right: 4px; }
  .ok { color: #4caf50; }
  .missing { color: #f44336; }
  code { background: #333; padding: 2px 6px; border-radius: 4px; font-size: 0.9em; }
  .warning { font-size: 0.85em; color: #888; margin-bottom: 24px; }
  .actions { display: flex; justify-content: flex-end; gap: 12px; }
  .btn-cancel {
    background: #333;
    color: #ccc;
    border: 1px solid #555;
    border-radius: 6px;
    padding: 8px 20px;
    cursor: pointer;
  }
  .btn-cancel:hover { background: #444; }
  .btn-confirm {
    background: #4a9eff;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 8px 22px;
    font-weight: 600;
    cursor: pointer;
  }
  .btn-confirm:hover:not(:disabled) { background: #3a8eef; }
  .btn-confirm:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
