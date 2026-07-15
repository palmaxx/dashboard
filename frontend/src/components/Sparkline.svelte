<script>
  export let values = []
  export let compareValues = []
  export let color = 'var(--cyan)'
  export let compareColor = 'var(--orange)'
  export let min = 0
  export let max = 100
  export let topLabel = '100%'
  export let middleLabel = '50%'
  export let bottomLabel = '0%'
  export let startLabel = '−5m'
  export let endLabel = 'Now'
  export let ariaLabel = 'Metric history over the last five minutes'
  export let fill = false

  const WIDTH = 640
  const HEIGHT = 100
  const PAD = 6

  function buildPoints(series) {
    if (!Array.isArray(series) || series.length < 2) return []
    const span = Math.max(max - min, Number.EPSILON)
    return series
      .map((value, index) => ({
        value,
        x: (index / (series.length - 1)) * WIDTH
      }))
      .filter(point => Number.isFinite(point.value))
      .map(point => ({
        x: point.x,
        y: HEIGHT - PAD - ((Math.min(max, Math.max(min, point.value)) - min) / span) * (HEIGHT - PAD * 2)
      }))
  }

  $: points = buildPoints(values)
  $: comparePoints = buildPoints(compareValues)
  $: polyline = points.map(point => point.x + ',' + point.y).join(' ')
  $: comparePolyline = comparePoints.map(point => point.x + ',' + point.y).join(' ')
  $: areaPath = points.length > 1
    ? '0,' + HEIGHT + ' ' + polyline + ' ' + WIDTH + ',' + HEIGHT
    : ''
</script>

<div class="chart" class:empty={points.length < 2}>
  <div class="axis-labels" aria-hidden="true">
    <span>{topLabel}</span>
    <span>{middleLabel}</span>
    <span>{bottomLabel}</span>
  </div>

  <svg viewBox="0 0 {WIDTH} {HEIGHT}" preserveAspectRatio="none" role="img" aria-label={ariaLabel}>
    <line x1="0" y1={PAD} x2={WIDTH} y2={PAD}></line>
    <line x1="0" y1={HEIGHT / 2} x2={WIDTH} y2={HEIGHT / 2}></line>
    <line x1="0" y1={HEIGHT - PAD} x2={WIDTH} y2={HEIGHT - PAD}></line>

    {#if points.length > 1}
      {#if fill}
        <polygon points={areaPath} fill={color} opacity="0.12"></polygon>
      {/if}
      <polyline
        points={polyline}
        fill="none"
        stroke={color}
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
        vector-effect="non-scaling-stroke"
      ></polyline>
    {/if}

    {#if comparePoints.length > 1}
      <polyline
        points={comparePolyline}
        fill="none"
        stroke={compareColor}
        stroke-width="2.25"
        stroke-linecap="round"
        stroke-linejoin="round"
        vector-effect="non-scaling-stroke"
      ></polyline>
    {/if}
  </svg>

  <div class="time-labels" aria-hidden="true">
    <span>{startLabel}</span>
    {#if points.length < 2}
      <span class="history-note">Building history</span>
    {/if}
    <span>{endLabel}</span>
  </div>
</div>

<style>
  .chart {
    position: relative;
    min-width: 0;
    height: 8.5rem;
    padding: var(--sp-2) 3rem 1.5rem 0;
  }

  svg {
    display: block;
    width: 100%;
    height: 100%;
    overflow: visible;
  }

  line {
    stroke: var(--line);
    stroke-width: 1;
    vector-effect: non-scaling-stroke;
  }

  .axis-labels {
    position: absolute;
    top: var(--sp-2);
    right: 0;
    bottom: 1.5rem;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
    line-height: 1;
    text-align: right;
  }

  .time-labels {
    position: absolute;
    right: 3rem;
    bottom: 0;
    left: 0;
    display: flex;
    justify-content: space-between;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }

  .history-note {
    color: var(--text-secondary);
    font-family: var(--font-sans);
  }
</style>
