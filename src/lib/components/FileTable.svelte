<script lang="ts">
  import { fileList } from '../stores';
  import type { FileEntry } from '../types';

  function toggleAll(e: Event) {
    const checked = (e.target as HTMLInputElement).checked;
    fileList.update(list => list.map(f => ({ ...f, selected: checked })));
  }

  function toggleOne(id: string) {
    fileList.update(list =>
      list.map(f => f.id === id ? { ...f, selected: !f.selected } : f)
    );
  }

  function statusLabel(entry: FileEntry): string {
    switch (entry.status) {
      case 'ok': return 'OK';
      case 'conflict': return 'Conflict';
      case 'missing_metadata': return 'No metadata';
    }
  }

  $: allSelected = $fileList.every(f => f.selected);
  $: someSelected = $fileList.some(f => f.selected);
</script>

<div class="table-wrapper">
  <table>
    <thead>
      <tr>
        <th class="col-check">
          <input
            type="checkbox"
            checked={allSelected}
            indeterminate={someSelected && !allSelected}
            on:change={toggleAll}
          />
        </th>
        <th>Original name</th>
        <th>New name</th>
        <th>Date / Time</th>
        <th>Camera</th>
        <th>Status</th>
      </tr>
    </thead>
    <tbody>
      {#each $fileList as entry (entry.id)}
        <tr class="row-{entry.status}" class:deselected={!entry.selected}>
          <td class="col-check">
            <input
              type="checkbox"
              checked={entry.selected}
              on:change={() => toggleOne(entry.id)}
            />
          </td>
          <td class="name" title={entry.original_path}>{entry.original_name}</td>
          <td class="name proposed">
            {entry.proposed_name ?? '—'}
          </td>
          <td>{entry.datetime_str ?? '—'}</td>
          <td>{entry.camera_model ?? '—'}</td>
          <td>
            <span class="badge badge-{entry.status}">{statusLabel(entry)}</span>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-wrapper {
    overflow-y: auto;
    flex: 1;
    border: 1px solid #333;
    border-radius: 6px;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875em;
  }

  thead {
    position: sticky;
    top: 0;
    background: #1e1e1e;
    z-index: 1;
  }

  th {
    padding: 10px 12px;
    text-align: left;
    color: #888;
    font-weight: 600;
    border-bottom: 1px solid #333;
  }

  td {
    padding: 8px 12px;
    border-bottom: 1px solid #222;
    color: #ccc;
    vertical-align: middle;
  }

  tr:last-child td { border-bottom: none; }

  .col-check { width: 40px; text-align: center; }

  .name {
    font-family: monospace;
    max-width: 240px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .proposed { color: #4a9eff; }

  .deselected td { opacity: 0.4; }

  .row-ok:hover td { background: #1a2a1a; }
  .row-conflict:hover td { background: #2a1f0a; }
  .row-missing_metadata:hover td { background: #2a1a1a; }

  .badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 0.8em;
    font-weight: 600;
  }

  .badge-ok { background: #1b3a1b; color: #4caf50; }
  .badge-conflict { background: #3a2a00; color: #ff9800; }
  .badge-missing_metadata { background: #3a1a1a; color: #f44336; }
</style>
