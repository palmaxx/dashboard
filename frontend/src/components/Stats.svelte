<script>
  import { fade } from 'svelte/transition';

  export let hardware = null;
  export let status = 'loading';
  export let lastUpdated = null;
  export let showStatusBar = true;
  export let compact = false;

  const unavailable = 'Unavailable';
  const memorySegments = Array.from({ length: 24 }, (_, index) => index);
  const chartWidth = 180;
  const chartHeight = 56;
  const chartPadX = 4;
  const chartPadY = 7;

  function numberOrNull(value) {
    return typeof value === 'number' && Number.isFinite(value) ? value : null;
  }

  function clampPercent(value) {
    const number = numberOrNull(value);
    if (number === null) return null;
    return Math.min(100, Math.max(0, number));
  }

  function percentValue(value) {
    return clampPercent(value) ?? 0;
  }

  function formatPercent(value) {
    const percent = clampPercent(value);
    return percent === null ? unavailable : `${Math.round(percent)}%`;
  }

  function formatGb(value) {
    const number = numberOrNull(value);
    return number === null ? unavailable : `${number.toFixed(1)} GB`;
  }

  function formatSpeed(value) {
    const number = numberOrNull(value);
    return number === null ? unavailable : `${number.toFixed(2)} MB/s`;
  }

  function formatTemperature(value) {
    const number = numberOrNull(value);
    return number === null ? unavailable : `${Math.round(number)}°C`;
  }

  function formatFrequency(value) {
    const number = numberOrNull(value);
    return number === null ? unavailable : `${number.toFixed(1)} GHz`;
  }

  function formatText(value) {
    return typeof value === 'string' && value.trim() ? value.trim() : unavailable;
  }

  function formatCpuSummary(cpu) {
    if (!cpu) return 'No CPU sample';

    const parts = [];
    if (typeof cpu.cores === 'number' && Number.isFinite(cpu.cores) && cpu.cores > 0) {
      parts.push(`${cpu.cores} cores`);
    }
    const speed = formatFrequency(cpu.speedGHz);
    if (speed !== unavailable) {
      parts.push(speed);
    }

    return parts.length ? parts.join(' - ') : 'CPU sample available';
  }

  function formatAvailableMemory(memory) {
    const total = numberOrNull(memory?.totalGB);
    const used = numberOrNull(memory?.usedGB);
    if (total === null || used === null) return unavailable;
    return formatGb(Math.max(0, total - used));
  }

  function formatVramSummary(gpu) {
    const vram = formatGb(gpu?.vramGB);
    return vram === unavailable ? 'VRAM unavailable' : `${vram} VRAM`;
  }

  function activeSegments(value) {
    return Math.round((percentValue(value) / 100) * memorySegments.length);
  }

  function historyValues(samples, key, percent = false) {
    return samples
      .map((sample) => (percent ? clampPercent(sample?.[key]) : numberOrNull(sample?.[key])))
      .filter((value) => value !== null);
  }

  function buildPolyline(values, minValue, maxValue) {
    if (!Array.isArray(values) || values.length < 2) return '';

    const usableMax = maxValue > minValue ? maxValue : minValue + 1;
    const xSpan = chartWidth - chartPadX * 2;
    const ySpan = chartHeight - chartPadY * 2;

    return values
      .map((value, index) => {
        const x = chartPadX + (index / (values.length - 1)) * xSpan;
        const normalized = Math.min(1, Math.max(0, (value - minValue) / (usableMax - minValue)));
        const y = chartHeight - chartPadY - normalized * ySpan;
        return `${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(' ');
  }

  function buildPercentTrace(samples, key) {
    return buildPolyline(historyValues(samples, key, true), 0, 100);
  }

  function networkScale(samples) {
    const values = [
      ...historyValues(samples, 'downloadMBps'),
      ...historyValues(samples, 'uploadMBps'),
    ];
    const max = values.length ? Math.max(...values) : 0;
    return max > 0 ? max : 1;
  }

  function buildNetworkTrace(samples, key, maxValue) {
    return buildPolyline(historyValues(samples, key), 0, maxValue);
  }

  function areaPoints(trace) {
    return trace ? `${chartPadX},${chartHeight - chartPadY} ${trace} ${chartWidth - chartPadX},${chartHeight - chartPadY}` : '';
  }

  function titleCase(value) {
    if (!value) return unavailable;
    return `${value.slice(0, 1).toUpperCase()}${value.slice(1)}`;
  }

  $: online = status === 'online' && hardware;
  $: history = online && Array.isArray(hardware.history) ? hardware.history : [];
  $: cpuUsage = online ? clampPercent(hardware.cpu?.usagePercent) : null;
  $: memoryUsage = online ? clampPercent(hardware.memory?.usagePercent) : null;
  $: gpuUsage = online ? clampPercent(hardware.gpu?.usagePercent) : null;
  $: networkStatus = online ? hardware.network?.status || 'offline' : '';
  $: cpuTrace = buildPercentTrace(history, 'cpuUsagePercent');
  $: gpuTrace = buildPercentTrace(history, 'gpuUsagePercent');
  $: netScale = networkScale(history);
  $: netDownTrace = buildNetworkTrace(history, 'downloadMBps', netScale);
  $: netUpTrace = buildNetworkTrace(history, 'uploadMBps', netScale);
  $: metrics = [
    {
      id: 'cpu',
      label: 'CPU',
      icon: 'fa-microchip',
      visual: 'line',
      value: cpuUsage,
      primary: formatPercent(cpuUsage),
      secondary: online ? formatText(hardware.cpu?.model) : 'Waiting for local daemon',
      readingMeta: online ? formatCpuSummary(hardware.cpu) : 'No CPU sample',
      detailA: 'Temp',
      detailAValue: online ? formatTemperature(hardware.cpu?.temperatureC) : unavailable,
      detailB: 'Clock',
      detailBValue: online ? formatFrequency(hardware.cpu?.speedGHz) : unavailable,
      trace: cpuTrace,
      traceLabel: online && history.length < 2 ? 'Collecting samples' : 'No CPU samples',
    },
    {
      id: 'memory',
      label: 'Memory',
      icon: 'fa-server',
      visual: 'segments',
      value: memoryUsage,
      primary: formatPercent(memoryUsage),
      secondary: online ? `${formatGb(hardware.memory?.usedGB)} / ${formatGb(hardware.memory?.totalGB)}` : 'No memory sample',
      readingMeta: online ? `${formatGb(hardware.memory?.usedGB)} used` : 'No memory sample',
      detailA: 'Used',
      detailAValue: online ? formatGb(hardware.memory?.usedGB) : unavailable,
      detailB: 'Available',
      detailBValue: online ? formatAvailableMemory(hardware.memory) : unavailable,
    },
    {
      id: 'gpu',
      label: 'GPU',
      icon: 'fa-desktop',
      visual: 'area',
      value: gpuUsage,
      primary: formatPercent(gpuUsage),
      secondary: online ? (formatText(hardware.gpu?.model) === unavailable ? 'GPU telemetry unavailable' : formatText(hardware.gpu?.model)) : 'No GPU sample',
      readingMeta: online ? formatVramSummary(hardware.gpu) : 'No GPU sample',
      detailA: 'Temp',
      detailAValue: online ? formatTemperature(hardware.gpu?.temperatureC) : unavailable,
      detailB: 'VRAM',
      detailBValue: online ? formatGb(hardware.gpu?.vramGB) : unavailable,
      trace: gpuTrace,
      areaTrace: areaPoints(gpuTrace),
      traceLabel: online && history.length < 2 ? 'Collecting samples' : 'No GPU telemetry',
    },
    {
      id: 'network',
      label: 'Network',
      icon: 'fa-wifi',
      visual: 'network',
      value: networkStatus === 'online' ? 100 : 0,
      primary: online ? titleCase(networkStatus) : unavailable,
      secondary: online ? formatText(hardware.network?.interfaceName) : 'No network sample',
      readingMeta: online ? formatText(hardware.network?.interfaceName) : 'No network sample',
      download: online ? formatSpeed(hardware.network?.downloadMBps) : unavailable,
      upload: online ? formatSpeed(hardware.network?.uploadMBps) : unavailable,
      detailA: 'Down',
      detailAValue: online ? formatSpeed(hardware.network?.downloadMBps) : unavailable,
      detailB: 'Up',
      detailBValue: online ? formatSpeed(hardware.network?.uploadMBps) : unavailable,
      traceDown: netDownTrace,
      traceUp: netUpTrace,
      traceLabel: online && history.length < 2 ? 'Collecting samples' : 'No network samples',
    },
  ];
</script>

<section class="stats-deck" class:compact>
  {#if showStatusBar && !compact && status !== 'online'}
    <div
      transition:fade={{ duration: 220 }}
      class="daemon-strip"
      class:online={status === 'online'}
      class:offline={status === 'offline'}
    >
      <div>
        <span></span>
        {#if status === 'online'}
          <strong>Local Daemon Connected</strong>
        {:else if status === 'offline'}
          <strong>Local Daemon Offline</strong>
        {:else}
          <strong>Connecting to Local Daemon</strong>
        {/if}
      </div>
      <p>
        {#if status === 'online' && lastUpdated}
          Last update {lastUpdated.toLocaleTimeString()}
        {:else if status === 'offline'}
          Run the Rust daemon for live system stats
        {:else}
          Waiting for first hardware snapshot
        {/if}
      </p>
    </div>
  {/if}

  {#if compact}
    <div class="module-header">
      <div>
        <p>System Telemetry</p>
        <h2>Hardware</h2>
      </div>
      <span class="module-status" class:online={status === 'online'} class:offline={status === 'offline'}>
        {status === 'online' ? 'Live' : status === 'offline' ? 'Offline' : 'Loading'}
      </span>
    </div>
  {/if}

  <div class="instrument-stack">
    {#each metrics as metric}
      <article class="instrument-row {metric.id}">
        <div class="instrument-icon">
          <i class="fa {metric.icon}" aria-hidden="true"></i>
        </div>

        <div class="instrument-name">
          <h3>{metric.label}</h3>
          <p>{metric.secondary}</p>
        </div>

        <div class="instrument-reading">
          {#if metric.id === 'network'}
            <div class="speed-pair">
              <span><i class="fa fa-arrow-down" aria-hidden="true"></i>{metric.download}</span>
              <span><i class="fa fa-arrow-up" aria-hidden="true"></i>{metric.upload}</span>
            </div>
          {:else}
            <strong>{metric.primary}</strong>
            <span>{metric.readingMeta}</span>
          {/if}
        </div>

        <div class="metric-visual {metric.visual}" aria-hidden="true">
          {#if metric.visual === 'segments'}
            {#each memorySegments as segment}
              <span class:active={segment < activeSegments(metric.value)}></span>
            {/each}
          {:else if metric.visual === 'network'}
            {#if metric.traceDown || metric.traceUp}
              <svg viewBox="0 0 180 56" preserveAspectRatio="none">
                {#if metric.traceUp}
                  <polyline class="trace-up" points={metric.traceUp}></polyline>
                {/if}
                {#if metric.traceDown}
                  <polyline class="trace-down" points={metric.traceDown}></polyline>
                {/if}
              </svg>
            {:else}
              <span class="trace-empty">{metric.traceLabel}</span>
            {/if}
          {:else if metric.trace}
            <svg viewBox="0 0 180 56" preserveAspectRatio="none">
              {#if metric.visual === 'area' && metric.areaTrace}
                <polygon points={metric.areaTrace}></polygon>
              {/if}
              <polyline points={metric.trace}></polyline>
            </svg>
          {:else}
            <span class="trace-empty">{metric.traceLabel}</span>
          {/if}
        </div>

        <dl class="instrument-details">
          <div>
            <dt>{metric.detailA}</dt>
            <dd>{metric.detailAValue}</dd>
          </div>
          <div>
            <dt>{metric.detailB}</dt>
            <dd title={metric.detailBValue}>{metric.detailBValue}</dd>
          </div>
        </dl>
      </article>
    {/each}
  </div>

  {#if !compact}
    <section class="storage-panel">
      <div class="module-header compact-header">
        <div>
          <p>Local Drives</p>
          <h2>Storage Volumes</h2>
        </div>
        <span>{online && hardware.storage ? hardware.storage.length : 0} volumes</span>
      </div>

      <div class="storage-list">
        {#if online && hardware.storage?.length}
          {#each hardware.storage as disk}
            <article class="storage-row">
              <i class="fa fa-hdd-o" aria-hidden="true"></i>
              <div class="storage-name">
                <strong title={disk.name}>{formatText(disk.name)}</strong>
                <span title={disk.mount}>{formatText(disk.mount || disk.type)}</span>
              </div>
              <div class="storage-meter">
                <span style={`width: ${percentValue(disk.usagePercent)}%`}></span>
              </div>
              <p>{formatGb(disk.usedGB)} / {formatGb(disk.totalGB)}</p>
              <strong>{formatPercent(disk.usagePercent)}</strong>
              <small>{formatText(disk.type)}</small>
            </article>
          {/each}
        {:else}
          <div class="storage-empty">
            <i class="fa fa-hdd-o" aria-hidden="true"></i>
            <p>{online ? 'No storage volumes reported by the daemon.' : 'Storage appears after the local daemon reports a sample.'}</p>
          </div>
        {/if}
      </div>
    </section>
  {/if}
</section>
