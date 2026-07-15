<script>
  export let storage = []
  export let status = 'loading'

  $: online = status === 'online'

  function pct(v) { return typeof v === 'number' ? `${Math.round(v)}%` : '—' }
  function gb(v) { return typeof v === 'number' ? `${v.toFixed(1)} GB` : '—' }
</script>

<section class="card">
  <div class="card-header">
    <div>
      <h2>Storage</h2>
      <p>Local drive capacity</p>
    </div>
    <span class="vol-count">{storage.length} volumes</span>
  </div>

  {#if online && storage.length > 0}
    <div class="vol-list">
      {#each storage as disk}
        <div class="vol-row">
          <div class="vol-info">
            <strong title={disk.name}>{disk.name || disk.mount}</strong>
            <span>{disk.mount} · {disk.type}</span>
          </div>
          <div class="vol-bar">
            <div class="bar-track">
              <div class="bar-fill" style="width: {Math.min(100, disk.usagePercent)}%"></div>
            </div>
          </div>
          <span class="vol-usage mono">{gb(disk.usedGB)} / {gb(disk.totalGB)}</span>
          <span class="vol-pct mono">{pct(disk.usagePercent)}</span>
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <p>{online ? 'No storage volumes reported.' : 'Storage info appears when the daemon is running.'}</p>
    </div>
  {/if}
</section>

<style>
  .card {
    background: var(--surface);
  }

  .card-header {
    min-height: 4.5rem;
    background: var(--surface-raised);
  }

  .card-header p {
    margin-top: var(--sp-1);
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .vol-count {
    padding: var(--sp-1) var(--sp-2);
    border: 0.0625rem solid var(--line);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
  }

  .vol-list {
    display: flex;
    flex-direction: column;
  }

  .vol-row {
    display: grid;
    min-height: 4.25rem;
    grid-template-columns: minmax(8rem, 0.9fr) minmax(8rem, 1.7fr) auto auto;
    align-items: center;
    gap: var(--sp-4);
    padding: var(--sp-3) var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
  }

  .vol-row:last-child { border-bottom: none; }

  .vol-info {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .vol-info strong {
    overflow: hidden;
    color: var(--text-primary);
    font-size: 0.875rem;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .vol-info span {
    color: var(--text-secondary);
    font-size: 0.75rem;
  }

  .vol-bar {
    flex: 1;
    min-width: 80px;
  }

  .bar-track {
    height: 0.5rem;
    overflow: hidden;
    border-radius: var(--radius-sm);
    background: var(--line);
  }

  .bar-fill {
    height: 100%;
    background: var(--accent);
  }

  .vol-usage {
    color: var(--text-secondary);
    font-size: 0.75rem;
    white-space: nowrap;
  }

  .vol-pct {
    color: var(--text-primary);
    min-width: 2.5rem;
    font-size: 0.875rem;
    font-weight: 650;
    text-align: right;
  }

  @media (max-width: 40rem) {
    .vol-row {
      grid-template-columns: minmax(0, 1fr) auto;
    }

    .vol-bar {
      grid-column: 1 / -1;
      grid-row: 2;
    }

    .vol-usage {
      display: none;
    }
  }
</style>
