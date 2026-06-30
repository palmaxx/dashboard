<script>
  export let values = []
  export let color = 'var(--accent)'
  export let fill = false
  export let height = 48

  const W = 160
  const PAD = 4

  $: points = buildPoints(values)
  $: polyline = points.map(p => `${p.x},${p.y}`).join(' ')
  $: areaPath = points.length > 1
    ? `${PAD},${height - PAD} ${polyline} ${W - PAD},${height - PAD}`
    : ''

  function buildPoints(vals) {
    if (!vals || vals.length < 2) return []
    const min = Math.min(...vals)
    const max = Math.max(...vals)
    const range = max > min ? max - min : 1
    const xSpan = W - PAD * 2
    const ySpan = height - PAD * 2

    return vals.map((v, i) => ({
      x: PAD + (i / (vals.length - 1)) * xSpan,
      y: height - PAD - ((v - min) / range) * ySpan,
    }))
  }
</script>

<svg viewBox="0 0 {W} {height}" preserveAspectRatio="none" class="sparkline">
  {#if points.length > 1}
    {#if fill}
      <polygon points={areaPath} fill={color} opacity="0.15" />
    {/if}
    <polyline points={polyline} fill="none" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round" vector-effect="non-scaling-stroke" />
  {/if}
</svg>

<style>
  .sparkline {
    display: block;
    width: 100%;
    height: 100%;
  }
</style>
