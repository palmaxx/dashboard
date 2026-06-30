<script>
  import Sparkline from './Sparkline.svelte'

  export let label = ''
  export let icon = ''
  export let value = ''
  export let subtitle = ''
  export let detail1Label = ''
  export let detail1Value = ''
  export let detail2Label = ''
  export let detail2Value = ''
  export let accentColor = 'var(--accent)'
  export let sparklineValues = []
  export let sparklineFill = false
  export let segments = null
  export let isNetwork = false
  export let downloadSpeed = ''
  export let uploadSpeed = ''
</script>

<article class="metric-card card">
  <div class="metric-head">
    <span class="metric-icon">{icon}</span>
    <div class="metric-info">
      <h3>{label}</h3>
      <p class="subtitle" title={subtitle}>{subtitle}</p>
    </div>
  </div>

  <div class="metric-body">
    {#if isNetwork}
      <div class="speed-stack mono">
        <span class="speed down">↓ {downloadSpeed}</span>
        <span class="speed up">↑ {uploadSpeed}</span>
      </div>
    {:else}
      <span class="metric-value mono">{value}</span>
    {/if}

    <div class="visual">
      {#if segments !== null}
        <div class="segment-bar">
          {#each Array(20) as _, i}
            <span class="seg" class:active={i < Math.round((segments / 100) * 20)} style="--seg-color: {accentColor}"></span>
          {/each}
        </div>
      {:else if sparklineValues.length > 1}
        <Sparkline values={sparklineValues} color={accentColor} fill={sparklineFill} />
      {:else}
        <span class="no-data">Collecting…</span>
      {/if}
    </div>
  </div>

  <div class="metric-details">
    <div class="detail">
      <span class="detail-label">{detail1Label}</span>
      <span class="detail-value mono">{detail1Value}</span>
    </div>
    <div class="detail">
      <span class="detail-label">{detail2Label}</span>
      <span class="detail-value mono">{detail2Value}</span>
    </div>
  </div>
</article>

<style>
  .metric-card {
    display: flex;
    flex-direction: column;
    gap: var(--sp-3);
    padding: var(--sp-4);
  }

  .metric-head {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
  }

  .metric-icon {
    display: grid;
    width: 36px;
    height: 36px;
    place-items: center;
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    font-size: 18px;
    flex-shrink: 0;
  }

  .metric-info h3 {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .subtitle {
    font-size: 11px;
    color: var(--text-secondary);
    margin: 2px 0 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 180px;
  }

  .metric-body {
    display: flex;
    align-items: center;
    gap: var(--sp-4);
    min-height: 48px;
  }

  .metric-value {
    font-size: 28px;
    font-weight: 500;
    color: var(--text-primary);
    line-height: 1;
    flex-shrink: 0;
  }

  .visual {
    flex: 1;
    min-width: 0;
    height: 48px;
    display: flex;
    align-items: center;
  }

  .speed-stack {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
    font-size: 14px;
    font-weight: 600;
  }

  .speed.down { color: var(--accent); }
  .speed.up { color: var(--green); }

  .segment-bar {
    display: flex;
    gap: 3px;
    align-items: center;
    height: 28px;
    width: 100%;
  }

  .seg {
    flex: 1;
    height: 22px;
    border-radius: 3px;
    background: rgba(255,255,255,0.06);
    transition: background 300ms ease;
  }

  .seg.active {
    background: var(--seg-color, var(--accent));
    box-shadow: 0 0 8px color-mix(in srgb, var(--seg-color, var(--accent)) 40%, transparent);
  }

  .no-data {
    font-size: 12px;
    color: var(--text-tertiary);
  }

  .metric-details {
    display: flex;
    gap: var(--sp-4);
    padding-top: var(--sp-2);
    border-top: 1px solid var(--glass-border);
  }

  .detail {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .detail-label {
    font-size: 11px;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .detail-value {
    font-size: 13px;
    color: var(--text-secondary);
  }
</style>
