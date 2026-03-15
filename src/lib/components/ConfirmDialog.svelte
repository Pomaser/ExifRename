<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { fileList } from '../stores';

  export let show = false;

  const dispatch = createEventDispatcher<{ confirm: string[]; cancel: void }>();

  $: selectedEntries = $fileList.filter(f => f.selected);
  $: toRename = selectedEntries.filter(f => f.status === 'ok' || f.status === 'conflict').length;
  $: toNoExif = selectedEntries.filter(f => f.status === 'missing_metadata').length;

  function confirm() {
    dispatch('confirm', selectedEntries.map(f => f.id));
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
        <p><span class="num ok">{toRename}</span> file{toRename !== 1 ? 's' : ''} will be renamed</p>
        {#if toNoExif > 0}
          <p><span class="num missing">{toNoExif}</span> file{toNoExif !== 1 ? 's' : ''} will be moved to <code>NoExif/</code></p>
        {/if}
      </div>
      <p class="note">Renaming can be reversed using the Undo feature.</p>
      <div class="actions">
        <button class="btn-cancel" on:click={cancel}>Cancel</button>
        <button class="btn-confirm" on:click={confirm} disabled={selectedEntries.length === 0}>Rename</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }
  .dialog {
    background: #242424;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 24px 28px;
    min-width: 360px;
    max-width: 480px;
  }
  h2 { margin: 0 0 16px; color: #e0e0e0; font-size: 1.1em; font-weight: 600; }
  .summary { margin-bottom: 14px; }
  .summary p { margin: 5px 0; color: #aaa; font-size: 0.9em; }
  .num { font-weight: 700; margin-right: 4px; }
  .ok      { color: #5ecf5e; }
  .missing { color: #e05050; }
  code { background: #1a1a1a; padding: 1px 5px; border-radius: 3px; font-size: 0.88em; color: #aaa; }
  .note { font-size: 0.82em; color: #555; margin-bottom: 20px; }
  .actions { display: flex; justify-content: flex-end; gap: 10px; }
  .btn-cancel {
    background: #2a2a2a;
    color: #888;
    border: 1px solid #3a3a3a;
    border-radius: 5px;
    padding: 7px 18px;
    cursor: pointer;
    font-size: 0.88em;
    transition: background 0.15s;
  }
  .btn-cancel:hover { background: #333; }
  .btn-confirm {
    background: #1e5c1e;
    color: #7eda7e;
    border: 1px solid #2e7a2e;
    border-radius: 5px;
    padding: 7px 20px;
    font-weight: 600;
    font-size: 0.88em;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-confirm:hover:not(:disabled) { background: #256025; }
  .btn-confirm:disabled { opacity: 0.35; cursor: not-allowed; }
</style>
