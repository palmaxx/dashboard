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
    if (listener.processName && hasPid) return `${listener.processName} (${listener.processId})`
    if (listener.processName) return listener.processName
    if (hasPid) return `PID ${listener.processId}`
    return 'Unknown'
  }
</script>

<section class="card ports-card">
  <div class="card-header ports-header">
    <div>
      <p class="label">Local Network</p>
      <h2>Active Ports</h2>
    </div>
    <div class="header-actions">
      <span class="summary">{exposedCount} exposed / {listeners.length} total</span>
      <button class="btn btn-ghost" type="button" on:click={onRefresh} title="Refresh active ports">
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 12a9 9 0 0 1-15.5 6.2"/><path d="M3 12A9 9 0 0 1 18.5 5.8"/><path d="M18 2v4h4"/><path d="M6 22v-4H2"/></svg>
        <span>Refresh</span>
      </button>
    </div>
  </div>

  {#if status === 'offline' && listeners.length === 0}
    <div class="empty-state">
      <p>Port audit unavailable. Start the local daemon and refresh.</p>
    </div>
  {:else if status === 'loading' && listeners.length === 0}
    <div class="empty-state">
      <p>Scanning active ports...</p>
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
              <td class="proto">{listener.protocol}</td>
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
    overflow: hidden;
  }

  .ports-header {
    align-items: center;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
  }

  .summary {
    font-size: 12px;
    color: var(--text-secondary);
    padding: var(--sp-1) var(--sp-2);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    white-space: nowrap;
  }

  .ports-meta {
    display: flex;
    justify-content: space-between;
    gap: var(--sp-3);
    padding: var(--sp-3) var(--sp-5);
    color: var(--text-tertiary);
    font-size: 12px;
    border-bottom: 1px solid var(--glass-border);
  }

  .warn {
    color: var(--amber);
  }

  .table-wrap {
    overflow-x: auto;
  }

  table {
    width: 100%;
    min-width: 720px;
    border-collapse: collapse;
  }

  th,
  td {
    padding: var(--sp-3) var(--sp-5);
    border-bottom: 1px solid var(--glass-border);
    text-align: left;
    vertical-align: middle;
  }

  th {
    color: var(--text-tertiary);
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    background: var(--surface);
  }

  td {
    color: var(--text-secondary);
    font-size: 13px;
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  .badge {
    display: inline-flex;
    align-items: center;
    min-width: 94px;
    justify-content: center;
    padding: 3px var(--sp-2);
    border-radius: var(--radius-sm);
    font-size: 11px;
    font-weight: 700;
    white-space: nowrap;
  }

  .badge.lan {
    color: var(--red);
    background: rgba(248, 113, 113, 0.12);
    border: 1px solid rgba(248, 113, 113, 0.24);
  }

  .badge.local {
    color: var(--green);
    background: rgba(74, 222, 128, 0.1);
    border: 1px solid rgba(74, 222, 128, 0.22);
  }

  .badge.unknown {
    color: var(--amber);
    background: rgba(251, 191, 36, 0.1);
    border: 1px solid rgba(251, 191, 36, 0.22);
  }

  .address,
  .process {
    max-width: 260px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .port {
    color: var(--text-primary);
    font-weight: 700;
  }

  .proto {
    color: var(--text-primary);
    text-transform: uppercase;
    font-size: 12px;
    font-weight: 700;
  }

  @media (max-width: 700px) {
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
