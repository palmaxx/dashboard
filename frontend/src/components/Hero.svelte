<script>
  import { onMount } from 'svelte'

  export let daemonStatus = 'loading'
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

<section class="hero" aria-label="Time and dashboard status">
  <div class="hero-top glass-chrome">
    <a href="./index.html" class="brand" aria-label="Sleek Dashboard home">
      <span class="brand-mark" aria-hidden="true">SD</span>
      <span>Sleek Dashboard</span>
    </a>

    <div class="daemon-status" aria-live="polite">
      <span class="status-dot" class:online={daemonStatus === 'online'} class:offline={daemonStatus === 'offline'}></span>
      <span>
        {#if daemonStatus === 'online'}
          Local daemon connected
        {:else if daemonStatus === 'offline'}
          Local daemon offline
        {:else}
          Connecting to local daemon
        {/if}
      </span>
    </div>

    <div class="hero-actions" aria-label="Wallpaper controls">
      <button class="hero-button" type="button" on:click={onWallpaperClick} aria-label="Personalize dashboard" title="Personalize dashboard">
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
  </div>

  <div class="hero-content">
    <p class="greeting">
      <span class="status-dot" class:online={daemonStatus === 'online'} class:offline={daemonStatus === 'offline'}></span>
      {greeting}
    </p>
    <h1 class="clock">{time}</h1>
    <p class="date">{date}</p>
  </div>
</section>

<style>
  .hero {
    position: relative;
    z-index: 2;
    display: flex;
    width: min(96rem, 100%);
    min-height: 29rem;
    flex-direction: column;
    margin: 0 auto;
    padding: var(--sp-5) clamp(var(--sp-5), 3vw, var(--sp-10)) var(--sp-10);
    isolation: isolate;
  }

  .hero-top {
    display: grid;
    min-height: 4rem;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: var(--sp-4);
    padding: var(--sp-2) var(--sp-3);
    border-radius: var(--radius-lg);
  }

  .brand {
    display: inline-flex;
    min-width: 0;
    min-height: 2.75rem;
    align-items: center;
    justify-self: start;
    gap: var(--sp-3);
    color: white;
    font-size: 0.9375rem;
    font-weight: 650;
  }

  .brand-mark {
    display: grid;
    width: 2.25rem;
    height: 2.25rem;
    flex: 0 0 auto;
    place-items: center;
    border: 0.0625rem solid rgb(255 255 255 / 0.24);
    border-radius: var(--radius-sm);
    background: rgb(4 8 18 / 0.46);
    font-size: 0.75rem;
    font-weight: 750;
    letter-spacing: 0.04em;
  }

  .daemon-status {
    display: inline-flex;
    min-height: 2.5rem;
    align-items: center;
    justify-self: center;
    gap: var(--sp-2);
    padding: var(--sp-1) var(--sp-3);
    border-radius: var(--radius-sm);
    color: rgb(255 255 255 / 0.86);
    font-size: 0.8125rem;
    font-weight: 600;
  }

  .hero-actions {
    display: flex;
    justify-self: end;
    gap: var(--sp-1);
  }

  .hero-button {
    display: grid;
    width: 2.75rem;
    height: 2.75rem;
    place-items: center;
    border: 0.0625rem solid transparent;
    border-radius: var(--radius-md);
    color: rgb(255 255 255 / 0.84);
    transition: background-color 180ms cubic-bezier(0.22, 1, 0.36, 1),
      border-color 180ms cubic-bezier(0.22, 1, 0.36, 1),
      color 180ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .hero-button:hover {
    border-color: rgb(255 255 255 / 0.2);
    background: rgb(255 255 255 / 0.1);
    color: white;
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
    flex: 1;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    padding: var(--sp-10) clamp(var(--sp-3), 4vw, 4rem) var(--sp-12);
    color: white;
    text-align: left;
  }

  .greeting {
    display: inline-flex;
    align-items: center;
    gap: var(--sp-2);
    margin-bottom: var(--sp-2);
    font-size: 1rem;
    font-weight: 650;
    text-shadow: 0 0.125rem 0.25rem rgb(0 0 0 / 0.72);
  }

  .clock {
    color: white;
    font-family: var(--font-display);
    font-size: 6rem;
    font-variant-numeric: tabular-nums;
    font-weight: 400;
    letter-spacing: -0.035em;
    line-height: 0.9;
    text-shadow: 0 0.25rem 0.5rem rgb(0 0 0 / 0.52);
    text-wrap: balance;
  }

  .date {
    margin-top: var(--sp-3);
    font-size: 1.125rem;
    font-weight: 650;
    text-shadow: 0 0.125rem 0.25rem rgb(0 0 0 / 0.72);
  }

  @media (max-width: 47.5rem) {
    .hero {
      min-height: 25rem;
      padding: var(--sp-4);
      padding-bottom: var(--sp-8);
    }

    .hero-top {
      grid-template-columns: 1fr auto;
    }

    .daemon-status {
      grid-column: 1 / -1;
      grid-row: 2;
      width: 100%;
      min-height: 2rem;
      justify-content: flex-start;
      padding: 0 var(--sp-2) var(--sp-1);
    }

    .hero-actions {
      grid-column: 2;
      grid-row: 1;
    }

    .hero-content {
      padding: var(--sp-8) var(--sp-2) var(--sp-10);
    }

    .clock {
      font-size: 4.75rem;
    }

    .date {
      font-size: 1rem;
    }
  }

  @media (max-width: 24rem) {
    .brand > span:last-child {
      display: none;
    }

    .clock {
      font-size: 4.25rem;
    }
  }
</style>
