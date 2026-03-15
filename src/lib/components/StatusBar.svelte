<script lang="ts">
  import { scanResult } from '../stores';
</script>

{#if $scanResult}
  <div class="status-bar">
    <span>Total: <strong>{$scanResult.total}</strong></span>
    <span class="ok">OK: <strong>{$scanResult.ok_count}</strong></span>
    <span class="conflict">Conflicts: <strong>{$scanResult.conflict_count}</strong></span>
    <span class="missing">No metadata: <strong>{$scanResult.missing_count}</strong></span>
  </div>
{/if}

<style>
  .status-bar {
    display: flex;
    gap: 20px;
    padding: 6px 2px;
    font-size: 0.82em;
    color: #555;
    flex-shrink: 0;
  }
  strong { font-weight: 700; }
  .ok       { color: #4a8c4a; }
  .conflict { color: #8c6a1a; }
  .missing  { color: #8c3a3a; }
</style>
