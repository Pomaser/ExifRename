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

  $: allSelected = $fileList.length > 0 && $fileList.every(f => f.selected);
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
        <tr class:deselected={!entry.selected}>
          <td class="col-check">
            <input
              type="checkbox"
              checked={entry.selected}
              on:change={() => toggleOne(entry.id)}
            />
          </td>
          <td class="name" title={entry.original_path}>{entry.original_name}</td>
          <td class="name proposed" class:no-meta={entry.status === 'missing_metadata'}>
            {entry.proposed_name ?? '—'}
          </td>
          <td class="muted">{entry.datetime_str ?? '—'}</td>
          <td class="muted">{entry.camera_model ?? '—'}</td>
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
    border: 1px solid #2e2e2e;
    border-radius: 6px;
    background: #1e1e1e;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.85em;
  }

  thead {
    position: sticky;
    top: 0;
    background: #242424;
    z-index: 1;
  }

  th {
    padding: 8px 12px;
    text-align: left;
    color: #666;
    font-weight: 600;
    font-size: 0.82em;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    border-bottom: 1px solid #2e2e2e;
  }

  td {
    padding: 7px 12px;
    border-bottom: 1px solid #242424;
    color: #c8c8c8;
    vertical-align: middle;
  }

  tr:last-child td { border-bottom: none; }
  tr:hover td { background: #252525; }

  .col-check { width: 38px; text-align: center; }

  .name {
    font-family: monospace;
    font-size: 0.95em;
    max-width: 220px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .proposed { color: #5aabf0; }
  .no-meta { color: #555; }
  .muted { color: #666; }

  .deselected td { opacity: 0.35; }

  .badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.78em;
    font-weight: 700;
    letter-spacing: 0.03em;
  }

  .badge-ok               { background: #0d2e0d; color: #5ecf5e; border: 1px solid #1a4a1a; }
  .badge-conflict         { background: #2e1f00; color: #e0931a; border: 1px solid #4a3500; }
  .badge-missing_metadata { background: #2e0d0d; color: #e05050; border: 1px solid #4a1a1a; }
</style>
