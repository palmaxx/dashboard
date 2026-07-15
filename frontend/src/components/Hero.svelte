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
    const hour = now.getHours()
    greeting = hour < 5 ? 'Good night' : hour < 12 ? 'Good morning' : hour < 17 ? 'Good afternoon' : hour < 22 ? 'Good evening' : 'Good night'
  }

  onMount(() => {
    tick()
    const id = setInterval(tick, 1000)
    return () => clearInterval(id)
  })
</script>

<section
  class="hero"
  style="--wallpaper: url('{currentWallpaper}')"
  aria-label="Time and dashboard status"
>
  <header class="hero-top">
    <a href="./index.html" class="brand" aria-label="Sleek Dashboard home">
      <span class="brand-mark" aria-hidden="true">SD</span>
      <span>Sleek Dashboard</span>
    </a>

    <div class="hero-actions" aria-label="Wallpaper controls">
      <button class="hero-button" type="button" on:click={onWallpaperClick} aria-label="Choose wallpaper" title="Choose wallpaper">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <rect x="3" y="4" width="18" height="16" rx="2"></rect>
          <circle cx="8.5" cy="9" r="1.5"></circle>
          <path d="m3 16 5-5 4 4 2-2 7 7"></path>
        </svg>
      </button>
      <button class="hero-button" type="button" on:click={onShuffleClick} aria-label="Shuffle wallpaper" title="Shuffle wallpaper">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M16 3h5v5"></path>
          <path d="m4 20 16-16"></path>
          <path d="M21 16v5h-5"></path>
          <path d="m15 15 5 5"></path>
          <path d="m4 4 5 5"></path>
        </svg>
      </button>
    </div>
  </header>

  <div class="hero-content">
    <p class="greeting">
      <span class="status-dot" class:online={daemonStatus === 'online'} class:offline={daemonStatus === 'offline'}></span>
      {greeting}
    </p>
    <h1 class="clock">{time}</h1>
    <p class="date">{date}</p>
  </div>

  <div class="daemon-status" aria-live="polite">
    <span class="status-dot" class:online={daemonStatus === 'online'} class:offline={daemonStatus === 'offline'}></span>
    {#if daemonStatus === 'online'}
      Local daemon connected
    {:else if daemonStatus === 'offline'}
      Local daemon offline
    {:else}
      Connecting to local daemon
    {/if}
  </div>
</section>

<style>
  .hero {
    position: relative;
    display: flex;
    min-height: clamp(18.75rem, 36vh, 25rem);
    flex-direction: column;
    overflow: hidden;
    background-image:
      linear-gradient(90deg, rgb(3 7 15 / 0.92) 0%, rgb(5 10 22 / 0.68) 38%, rgb(5 10 22 / 0.08) 72%),
      linear-gradient(180deg, rgb(4 8 16 / 0.06) 35%, var(--bg) 112%),
      var(--wallpaper);
    background-position: center;
    background-size: cover;
    isolation: isolate;
  }

  .hero::after {
    position: absolute;
    right: 0;
    bottom: 0;
    left: 0;
    height: 0.5rem;
    background: var(--accent-strong);
    content: "";
    z-index: -1;
  }

  .hero-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-4);
    padding: var(--sp-5) clamp(var(--sp-5), 3vw, var(--sp-10));
  }

  .brand {
    display: inline-flex;
    min-height: 2.75rem;
    align-items: center;
    gap: var(--sp-3);
    color: white;
    font-size: 1rem;
    font-weight: 650;
  }

  .brand-mark {
    display: grid;
    width: 2.25rem;
    height: 2.25rem;
    place-items: center;
    border: 0.0625rem solid rgb(255 255 255 / 0.35);
    border-radius: var(--radius-sm);
    background: rgb(3 7 15 / 0.72);
    font-size: 0.75rem;
    font-weight: 750;
    letter-spacing: 0.05em;
  }

  .hero-actions {
    display: flex;
    gap: var(--sp-2);
  }

  .hero-button {
    display: grid;
    width: 2.75rem;
    height: 2.75rem;
    place-items: center;
    border: 0.0625rem solid rgb(255 255 255 / 0.3);
    border-radius: var(--radius-md);
    background: rgb(3 7 15 / 0.72);
    color: white;
    transition: background-color 180ms cubic-bezier(0.22, 1, 0.36, 1),
      border-color 180ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .hero-button:hover {
    border-color: white;
    background: var(--accent-strong);
  }

  .hero-button svg {
    width: 1.125rem;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 1.8;
  }

  .hero-content {
    display: flex;
    width: min(42rem, calc(100% - 2.5rem));
    flex: 1;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    align-self: flex-start;
    padding: 0 clamp(var(--sp-5), 6vw, 7.5rem) var(--sp-8);
    color: white;
    text-align: left;
  }

  .greeting {
    display: inline-flex;
    align-items: center;
    gap: var(--sp-2);
    margin-bottom: var(--sp-2);
    font-size: 1.125rem;
    font-weight: 600;
  }

  .clock {
    color: white;
    font-family: var(--font-display);
    font-size: clamp(4.75rem, 7vw, 6rem);
    font-variant-numeric: tabular-nums;
    font-weight: 400;
    letter-spacing: -0.035em;
    line-height: 0.9;
    text-shadow: 0 0.25rem 0.5rem rgb(0 0 0 / 0.45);
    text-wrap: balance;
  }

  .date {
    margin-top: var(--sp-3);
    font-size: 1.25rem;
    font-weight: 600;
    text-shadow: 0 0.125rem 0.25rem rgb(0 0 0 / 0.7);
  }

  .daemon-status {
    display: inline-flex;
    min-height: 2.75rem;
    align-items: center;
    align-self: flex-start;
    gap: var(--sp-2);
    margin: 0 clamp(var(--sp-5), 3vw, var(--sp-10)) var(--sp-4);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-sm);
    background: rgb(3 7 15 / 0.82);
    color: white;
    font-size: 0.875rem;
    font-weight: 600;
  }

  @media (max-width: 40rem) {
    .brand > span:last-child {
      display: none;
    }

    .hero-content {
      padding-right: var(--sp-5);
      padding-left: var(--sp-5);
    }

    .clock {
      font-size: 4.75rem;
    }

    .date {
      font-size: 1rem;
    }
  }
</style>
