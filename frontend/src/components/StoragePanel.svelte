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
      <p class="label">Local Drives</p>
      <h2>Storage</h2>
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
  .vol-count {
    font-size: 12px;
    color: var(--text-tertiary);
    padding: var(--sp-1) var(--sp-2);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
  }

  .vol-list {
    display: flex;
    flex-direction: column;
  }

  .vol-row {
    display: flex;
    align-items: center;
    gap: var(--sp-4);
    padding: var(--sp-3) var(--sp-5);
    border-bottom: 1px solid var(--glass-border);
  }

  .vol-row:last-child { border-bottom: none; }

  .vol-info {
    min-width: 120px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .vol-info strong {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .vol-info span {
    font-size: 11px;
    color: var(--text-tertiary);
  }

  .vol-bar {
    flex: 1;
    min-width: 80px;
  }

  .bar-track {
    height: 6px;
    border-radius: 3px;
    background: rgba(255,255,255,0.08);
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    border-radius: 3px;
    background: var(--accent);
    transition: width 400ms ease;
  }

  .vol-usage {
    font-size: 12px;
    color: var(--text-secondary);
    white-space: nowrap;
  }

  .vol-pct {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    min-width: 36px;
    text-align: right;
  }
</style>
