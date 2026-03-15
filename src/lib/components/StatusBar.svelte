<script lang="ts">
  import { scanResult } from '../stores';
</script>

{#if $scanResult}
  <div class="status-bar">
    <span class="total">Total: <strong>{$scanResult.total}</strong></span>
    <span class="ok">OK: <strong>{$scanResult.ok_count}</strong></span>
    <span class="conflict">Conflicts: <strong>{$scanResult.conflict_count}</strong></span>
    <span class="missing">No metadata: <strong>{$scanResult.missing_count}</strong></span>
  </div>
{/if}

<style>
  .status-bar {
    display: flex;
    gap: 24px;
    padding: 8px 4px;
    font-size: 0.9em;
    color: #aaa;
    border-top: 1px solid #333;
  }
  .total { color: #ccc; }
  .ok { color: #4caf50; }
  .conflict { color: #ff9800; }
  .missing { color: #f44336; }
  strong { font-weight: 700; }
</style>
