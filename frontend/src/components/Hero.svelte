<script>
  import { onMount } from 'svelte'

  export let daemonStatus = 'loading'
  export let currentWallpaper = ''
  export let onWallpaperClick = () => {}
  export let onShuffleClick = () => {}

  let time = ''
  let date = ''
  let greeting = ''

  function tick() {
    const now = new Date()
    time = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false })
    date = now.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' })
    const h = now.getHours()
    greeting = h < 5 ? 'Night' : h < 12 ? 'Morning' : h < 17 ? 'Afternoon' : h < 22 ? 'Evening' : 'Night'
  }

  onMount(() => {
    tick()
    const id = setInterval(tick, 1000)
    return () => clearInterval(id)
  })
</script>

<section
  class="hero"
  style="background-image: linear-gradient(180deg, rgba(10,15,20,0.15) 0%, rgba(10,15,20,0.5) 55%, var(--bg) 100%), url('{currentWallpaper}')"
>
  <div class="hero-top">
    <a href="./index.html" class="brand">
      <span class="brand-icon">SD</span>
      <span>Sleek Dashboard</span>
    </a>
    <div class="hero-actions">
      <button class="btn-icon" on:click={onWallpaperClick} aria-label="Change wallpaper" title="Change wallpaper">🖼</button>
      <button class="btn-icon" on:click={onShuffleClick} aria-label="Shuffle wallpaper" title="Shuffle wallpaper">⇄</button>
    </div>
  </div>

  <div class="hero-center">
    <div class="greeting-row">
      <span class="status-dot" class:online={daemonStatus === 'online'} class:offline={daemonStatus === 'offline'}></span>
      <span>Good {greeting}</span>
    </div>
    <h1 class="clock mono">{time}</h1>
    <div class="date-row">
      <span class="date-line"></span>
      <p>{date}</p>
      <span class="date-line"></span>
    </div>
  </div>

  <div class="daemon-status">
    <span class="status-dot" class:online={daemonStatus === 'online'} class:offline={daemonStatus === 'offline'}></span>
    <span>
      {#if daemonStatus === 'online'}Local Daemon Connected
      {:else if daemonStatus === 'offline'}Local Daemon Offline
      {:else}Connecting…{/if}
    </span>
  </div>
</section>

<style>
  .hero {
    position: relative;
    min-height: 320px;
    background-position: center;
    background-size: cover;
    display: flex;
    flex-direction: column;
  }

  .hero-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--sp-5) var(--sp-6);
    position: relative;
    z-index: 2;
  }

  .brand {
    display: inline-flex;
    align-items: center;
    gap: var(--sp-3);
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .brand-icon {
    display: grid;
    width: 30px;
    height: 30px;
    place-items: center;
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: var(--radius-sm);
    background: rgba(0,0,0,0.25);
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.08em;
  }

  .hero-actions {
    display: flex;
    gap: var(--sp-2);
  }

  .hero-center {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    position: relative;
    z-index: 2;
    text-align: center;
    padding: var(--sp-4);
  }

  .greeting-row {
    display: inline-flex;
    align-items: center;
    gap: var(--sp-2);
    color: rgba(255,255,255,0.75);
    font-size: 20px;
    font-weight: 500;
    margin-bottom: var(--sp-2);
  }

  .clock {
    font-size: 96px;
    font-weight: 400;
    line-height: 1;
    color: #fff;
    text-shadow: 0 12px 40px rgba(0,0,0,0.4);
    margin: 0;
  }

  .date-row {
    display: flex;
    align-items: center;
    gap: var(--sp-4);
    margin-top: var(--sp-2);
    color: rgba(255,255,255,0.8);
    font-size: 18px;
    font-weight: 500;
  }

  .date-row p { margin: 0; }

  .date-line {
    width: 48px;
    height: 1px;
    background: rgba(255,255,255,0.3);
  }

  .daemon-status {
    display: inline-flex;
    align-items: center;
    gap: var(--sp-2);
    padding: var(--sp-4) var(--sp-6);
    font-size: 13px;
    font-weight: 600;
    color: rgba(255,255,255,0.85);
    position: relative;
    z-index: 2;
  }
</style>
