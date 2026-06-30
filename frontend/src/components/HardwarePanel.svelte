<script>
  import MetricCard from './MetricCard.svelte'

  export let hardware = null
  export let status = 'loading'

  const NA = '—'

  function pct(v) { return typeof v === 'number' && isFinite(v) ? `${Math.round(v)}%` : NA }
  function gb(v) { return typeof v === 'number' && isFinite(v) ? `${v.toFixed(1)} GB` : NA }
  function speed(v) { return typeof v === 'number' && isFinite(v) ? `${v.toFixed(2)} MB/s` : NA }
  function temp(v) { return typeof v === 'number' && isFinite(v) ? `${Math.round(v)}°C` : NA }
  function freq(v) { return typeof v === 'number' && isFinite(v) ? `${v.toFixed(1)} GHz` : NA }
  function txt(v) { return typeof v === 'string' && v.trim() ? v.trim() : NA }

  $: online = status === 'online' && hardware
  $: history = online && Array.isArray(hardware.history) ? hardware.history : []
  $: cpuHistory = history.map(h => h.cpuUsagePercent)
  $: gpuHistory = history.map(h => h.gpuUsagePercent).filter(v => v != null)
</script>

<div class="hw-grid">
  <MetricCard
    label="CPU"
    icon="⚙"
    value={online ? pct(hardware.cpu.usagePercent) : NA}
    subtitle={online ? txt(hardware.cpu.model) : 'Waiting for daemon'}
    detail1Label="Temp"
    detail1Value={online ? temp(hardware.cpu.temperatureC) : NA}
    detail2Label="Clock"
    detail2Value={online ? freq(hardware.cpu.speedGHz) : NA}
    sparklineValues={cpuHistory}
  />
  <MetricCard
    label="Memory"
    icon="▦"
    value={online ? pct(hardware.memory.usagePercent) : NA}
    subtitle={online ? `${gb(hardware.memory.usedGB)} / ${gb(hardware.memory.totalGB)}` : 'No data'}
    detail1Label="Used"
    detail1Value={online ? gb(hardware.memory.usedGB) : NA}
    detail2Label="Free"
    detail2Value={online ? gb(hardware.memory.totalGB - hardware.memory.usedGB) : NA}
    segments={online ? hardware.memory.usagePercent : null}
    accentColor="var(--blue)"
  />
  <MetricCard
    label="GPU"
    icon="◈"
    value={online ? pct(hardware.gpu.usagePercent) : NA}
    subtitle={online ? txt(hardware.gpu.model) : 'No data'}
    detail1Label="Temp"
    detail1Value={online ? temp(hardware.gpu.temperatureC) : NA}
    detail2Label="VRAM"
    detail2Value={online ? gb(hardware.gpu.vramGB) : NA}
    sparklineValues={gpuHistory}
    sparklineFill={true}
    accentColor="var(--green)"
  />
  <MetricCard
    label="Network"
    icon="≋"
    value={online ? (hardware.network.status === 'online' ? 'Online' : 'Offline') : NA}
    subtitle={online ? txt(hardware.network.interfaceName) : 'No data'}
    detail1Label="Down"
    detail1Value={online ? speed(hardware.network.downloadMBps) : NA}
    detail2Label="Up"
    detail2Value={online ? speed(hardware.network.uploadMBps) : NA}
    isNetwork={true}
    downloadSpeed={online ? speed(hardware.network.downloadMBps) : NA}
    uploadSpeed={online ? speed(hardware.network.uploadMBps) : NA}
    accentColor="var(--amber)"
  />
</div>

<style>
  .hw-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--sp-3);
  }

  @media (max-width: 900px) {
    .hw-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
