<script>
  import Sparkline from './Sparkline.svelte'

  export let hardware = null
  export let status = 'loading'

  const NA = '—'

  function valid(value) {
    return typeof value === 'number' && Number.isFinite(value)
  }

  function pct(value) {
    return valid(value) ? Math.round(value) + '%' : NA
  }

  function gb(value) {
    return valid(value) ? value.toFixed(1) + ' GB' : NA
  }

  function temp(value) {
    return valid(value) ? Math.round(value) + '°C' : NA
  }

  function frequency(value) {
    return valid(value) ? value.toFixed(1) + ' GHz' : NA
  }

  function text(value) {
    return typeof value === 'string' && value.trim() ? value.trim() : NA
  }

  function rate(value) {
    if (!valid(value)) return NA
    const kilobytes = value * 1024
    if (kilobytes < 1) return Math.round(kilobytes * 1024) + ' B/s'
    if (value < 1) return kilobytes.toFixed(kilobytes >= 100 ? 0 : 1) + ' KB/s'
    return value.toFixed(value >= 10 ? 1 : 2) + ' MB/s'
  }

  function niceCeiling(value) {
    if (!valid(value) || value <= 0) return 0.01
    const power = Math.pow(10, Math.floor(Math.log10(value)))
    const fraction = value / power
    const nice = fraction <= 1 ? 1 : fraction <= 2 ? 2 : fraction <= 5 ? 5 : 10
    return nice * power
  }

  $: online = status === 'online' && hardware
  $: history = online && Array.isArray(hardware.history) ? hardware.history : []
  $: cpuHistory = history.map(sample => sample.cpuUsagePercent)
  $: gpuHistory = history.map(sample => sample.gpuUsagePercent).filter(valid)
  $: downloadHistory = history.map(sample => sample.downloadMBps)
  $: uploadHistory = history.map(sample => sample.uploadMBps)
  $: networkPeak = niceCeiling(Math.max(0, ...downloadHistory, ...uploadHistory))
  $: networkMid = networkPeak / 2
  $: memorySegments = online ? Math.round((hardware.memory.usagePercent / 100) * 20) : 0
</script>

<div class="telemetry-grid">
  <article class="telemetry-panel cpu-panel">
    <header class="panel-header">
      <div class="panel-title">
        <h2>CPU</h2>
        <p title={online ? hardware.cpu.model : ''}>{online ? text(hardware.cpu.model) : status === 'loading' ? 'Connecting to daemon' : 'Daemon offline'}</p>
      </div>
      <strong class="metric-value mono">{online ? pct(hardware.cpu.usagePercent) : NA}</strong>
    </header>

    <Sparkline
      values={cpuHistory}
      min={0}
      max={100}
      color="var(--cyan)"
      topLabel="100%"
      middleLabel="50%"
      bottomLabel="0%"
      ariaLabel="CPU usage over the last five minutes"
      fill={true}
    />

    <footer class="panel-details">
      <span><small>Temperature</small><strong class="mono">{online ? temp(hardware.cpu.temperatureC) : NA}</strong></span>
      <span><small>Clock</small><strong class="mono">{online ? frequency(hardware.cpu.speedGHz) : NA}</strong></span>
      <span><small>Cores</small><strong class="mono">{online ? hardware.cpu.cores : NA}</strong></span>
    </footer>
  </article>

  <article class="telemetry-panel memory-panel">
    <header class="panel-header compact">
      <div class="panel-title">
        <h2>Memory</h2>
        <p>{online ? gb(hardware.memory.usedGB) + ' of ' + gb(hardware.memory.totalGB) : 'No current reading'}</p>
      </div>
      <strong class="metric-value mono">{online ? pct(hardware.memory.usagePercent) : NA}</strong>
    </header>

    <div class="memory-meter" aria-label={online ? pct(hardware.memory.usagePercent) + ' memory used' : 'Memory unavailable'}>
      {#each Array(20) as _, index}
        <span class:active={index < memorySegments}></span>
      {/each}
    </div>

    <footer class="panel-details memory-details">
      <span><small>Used</small><strong class="mono">{online ? gb(hardware.memory.usedGB) : NA}</strong></span>
      <span><small>Free</small><strong class="mono">{online ? gb(hardware.memory.totalGB - hardware.memory.usedGB) : NA}</strong></span>
    </footer>
  </article>

  <article class="telemetry-panel network-panel">
    <header class="network-header">
      <div class="panel-title">
        <h2>Network</h2>
        <p>{online ? text(hardware.network.interfaceName) : 'No active interface'}</p>
      </div>
      <div class="network-rates">
        <span><i class="download-dot"></i><small>Download</small><strong class="mono">{online ? rate(hardware.network.downloadMBps) : NA}</strong></span>
        <span><i class="upload-dot"></i><small>Upload</small><strong class="mono">{online ? rate(hardware.network.uploadMBps) : NA}</strong></span>
      </div>
    </header>

    <Sparkline
      values={downloadHistory}
      compareValues={uploadHistory}
      min={0}
      max={networkPeak}
      color="var(--cyan)"
      compareColor="var(--orange)"
      topLabel={rate(networkPeak)}
      middleLabel={rate(networkMid)}
      bottomLabel="0 B/s"
      ariaLabel="Network download and upload rates over the last five minutes"
    />
  </article>

  <article class="telemetry-panel gpu-panel">
    <header class="panel-header compact">
      <div class="panel-title">
        <h2>GPU</h2>
        <p title={online ? hardware.gpu.model : ''}>{online ? text(hardware.gpu.model) : 'No current reading'}</p>
      </div>
      <strong class="metric-value mono">{online ? pct(hardware.gpu.usagePercent) : NA}</strong>
    </header>

    <Sparkline
      values={gpuHistory}
      min={0}
      max={100}
      color="var(--green)"
      topLabel="100%"
      middleLabel="50%"
      bottomLabel="0%"
      ariaLabel="GPU usage over the last five minutes"
    />

    <footer class="panel-details">
      <span><small>Temperature</small><strong class="mono">{online ? temp(hardware.gpu.temperatureC) : NA}</strong></span>
      <span><small>VRAM</small><strong class="mono">{online ? gb(hardware.gpu.vramGB) : NA}</strong></span>
    </footer>
  </article>
</div>

<style>
  .telemetry-grid {
    display: grid;
    grid-template-areas:
      "cpu memory"
      "network gpu";
    grid-template-columns: minmax(0, 1.7fr) minmax(16rem, 1fr);
    gap: var(--sp-4);
  }

  .telemetry-panel {
    min-width: 0;
    padding: var(--sp-5);
    border: 0.0625rem solid var(--line);
    border-radius: var(--radius-lg);
    background: var(--surface);
  }

  .cpu-panel {
    grid-area: cpu;
  }

  .memory-panel {
    display: flex;
    grid-area: memory;
    flex-direction: column;
  }

  .network-panel {
    grid-area: network;
    background: var(--surface-raised);
  }

  .gpu-panel {
    grid-area: gpu;
  }

  .panel-header,
  .network-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--sp-4);
  }

  .panel-header.compact {
    align-items: flex-start;
  }

  .panel-title {
    min-width: 0;
  }

  .panel-title h2 {
    color: var(--text-primary);
    font-size: 1.25rem;
    font-weight: 650;
    line-height: 1.2;
  }

  .panel-title p {
    margin-top: var(--sp-1);
    overflow: hidden;
    color: var(--text-secondary);
    font-size: 0.875rem;
    line-height: 1.35;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .metric-value {
    flex: 0 0 auto;
    color: var(--text-primary);
    font-size: 2rem;
    font-weight: 650;
    line-height: 1;
  }

  .panel-details {
    display: flex;
    flex-wrap: wrap;
    gap: var(--sp-6);
    margin-top: var(--sp-3);
    padding-top: var(--sp-3);
    border-top: 0.0625rem solid var(--line);
  }

  .panel-details span {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
  }

  .panel-details small,
  .network-rates small {
    color: var(--text-secondary);
    font-size: 0.75rem;
    line-height: 1.2;
  }

  .panel-details strong {
    color: var(--text-primary);
    font-size: 0.875rem;
    font-weight: 600;
  }

  .memory-meter {
    display: flex;
    height: 3.25rem;
    align-items: stretch;
    gap: var(--sp-1);
    margin: auto 0 var(--sp-5);
    padding-top: var(--sp-6);
  }

  .memory-meter span {
    min-width: 0.25rem;
    flex: 1;
    border-radius: 0.125rem;
    background: var(--line);
  }

  .memory-meter span.active {
    background: var(--accent);
  }

  .memory-details {
    margin-top: 0;
  }

  .network-header {
    align-items: center;
  }

  .network-rates {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: var(--sp-4);
  }

  .network-rates span {
    display: grid;
    grid-template-columns: auto auto;
    align-items: center;
    gap: 0 var(--sp-2);
  }

  .network-rates i {
    width: 0.5rem;
    height: 0.5rem;
    grid-row: span 2;
    border-radius: 50%;
  }

  .download-dot {
    background: var(--cyan);
  }

  .upload-dot {
    background: var(--orange);
  }

  .network-rates strong {
    color: var(--text-primary);
    font-size: 0.875rem;
    font-weight: 600;
  }

  @media (max-width: 50rem) {
    .telemetry-grid {
      grid-template-areas:
        "cpu"
        "memory"
        "network"
        "gpu";
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 34rem) {
    .telemetry-panel {
      padding: var(--sp-4);
    }

    .network-header {
      align-items: flex-start;
      flex-direction: column;
    }

    .network-rates {
      justify-content: flex-start;
    }
  }
</style>
