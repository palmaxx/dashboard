<script>
  export let snapshot = null
  export let status = 'idle'
  export let onRefresh = () => {}

  $: listeners = Array.isArray(snapshot?.listeners) ? snapshot.listeners : []
  $: exposedCount = listeners.filter(listener => listener.exposure === 'lan').length
  $: lastUpdated = formatTime(snapshot?.timestamp)

  function formatTime(value) {
    if (!value) return 'Never'
    const date = new Date(value)
    if (Number.isNaN(date.getTime())) return 'Unknown'
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })
  }

  function badgeText(exposure) {
    if (exposure === 'lan') return 'LAN exposed'
    if (exposure === 'local') return 'Local only'
    return 'Unknown'
  }

  function processText(listener) {
    const hasPid = typeof listener.processId === 'number'
    if (listener.processName && hasPid) return listener.processName + ' (' + listener.processId + ')'
    if (listener.processName) return listener.processName
    if (hasPid) return 'PID ' + listener.processId
    return 'Unknown'
  }
</script>

<section class="card ports-card">
  <header class="ports-header">
    <div>
      <h2>Active ports</h2>
      <p>Listening services on this machine</p>
    </div>
    <div class="header-actions">
      <span class="summary">{exposedCount} exposed / {listeners.length} total</span>
      <button class="btn btn-ghost" type="button" on:click={onRefresh} title="Refresh active ports">
        <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M21 12a9 9 0 0 1-15.5 6.2"></path><path d="M3 12A9 9 0 0 1 18.5 5.8"></path><path d="M18 2v4h4M6 22v-4H2"></path></svg>
        <span>Refresh</span>
      </button>
    </div>
  </header>

  {#if status === 'offline' && listeners.length === 0}
    <div class="empty-state">
      <p>Port audit unavailable. Start the local daemon and refresh.</p>
    </div>
  {:else if status === 'loading' && listeners.length === 0}
    <div class="empty-state">
      <p>Scanning active ports…</p>
    </div>
  {:else if listeners.length === 0}
    <div class="empty-state">
      <p>No listening ports reported.</p>
    </div>
  {:else}
    <div class="ports-meta">
      <span>Updated {lastUpdated}</span>
      {#if status === 'offline'}
        <span class="warn">Last refresh failed</span>
      {/if}
    </div>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th scope="col">Exposure</th>
            <th scope="col">Address</th>
            <th scope="col">Port</th>
            <th scope="col">Protocol</th>
            <th scope="col">Process</th>
          </tr>
        </thead>
        <tbody>
          {#each listeners as listener}
            <tr>
              <td><span class="badge {listener.exposure || 'unknown'}">{badgeText(listener.exposure)}</span></td>
              <td class="mono address" title={listener.localAddress}>{listener.localAddress}</td>
              <td class="mono port">{listener.port}</td>
              <td class="protocol">{listener.protocol}</td>
              <td class="process" title={processText(listener)}>{processText(listener)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>

<style>
  .ports-card {
    background: var(--surface);
  }

  .ports-header {
    display: flex;
    min-height: 4.75rem;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-4);
    padding: var(--sp-4) var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
    background: var(--surface-raised);
  }

  .ports-header h2 {
    font-size: 1.25rem;
    font-weight: 650;
  }

  .ports-header p {
    margin-top: var(--sp-1);
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
  }

  .header-actions svg {
    width: 1rem;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 2;
  }

  .summary {
    padding: var(--sp-1) var(--sp-2);
    border: 0.0625rem solid var(--line);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  .ports-meta {
    display: flex;
    justify-content: space-between;
    gap: var(--sp-3);
    padding: var(--sp-3) var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .warn {
    color: var(--amber);
  }

  .table-wrap {
    overflow-x: auto;
  }

  table {
    width: 100%;
    min-width: 45rem;
    border-collapse: collapse;
  }

  th,
  td {
    padding: var(--sp-3) var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
    text-align: left;
    vertical-align: middle;
  }

  th {
    background: var(--surface-raised);
    color: var(--text-secondary);
    font-size: 0.75rem;
    font-weight: 650;
  }

  td {
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  tbody tr:last-child td {
    border-bottom: 0;
  }

  .badge {
    display: inline-flex;
    min-width: 6rem;
    justify-content: center;
    padding: var(--sp-1) var(--sp-2);
    border: 0.0625rem solid currentColor;
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    font-size: 0.75rem;
    font-weight: 650;
    white-space: nowrap;
  }

  .badge.lan {
    color: var(--red);
  }

  .badge.local {
    color: var(--green);
  }

  .badge.unknown {
    color: var(--amber);
  }

  .address,
  .process {
    max-width: 16rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .port {
    color: var(--text-primary);
    font-weight: 700;
  }

  .protocol {
    color: var(--text-primary);
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
  }

  @media (max-width: 43.75rem) {
    .ports-header {
      align-items: flex-start;
      flex-direction: column;
    }

    .header-actions {
      width: 100%;
      justify-content: space-between;
    }
  }
</style>
