<script>
  import { onDestroy, onMount } from 'svelte'
  import { fetchPorts, fetchProjects, fetchSysinfo } from './lib/api.js'

  import BookmarksPanel from './components/BookmarksPanel.svelte'
  import HardwarePanel from './components/HardwarePanel.svelte'
  import Hero from './components/Hero.svelte'
  import PortsPanel from './components/PortsPanel.svelte'
  import ProjectsPanel from './components/ProjectsPanel.svelte'
  import StoragePanel from './components/StoragePanel.svelte'
  import WallpaperPicker from './components/WallpaperPicker.svelte'

  const WALLPAPERS = [
    { url: 'https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?q=80&w=2564&auto=format&fit=crop', name: 'Abstract Liquid' },
    { url: 'https://images.unsplash.com/photo-1550684848-fac1c5b4e853?q=80&w=2670&auto=format&fit=crop', name: 'Dark Geometry' },
    { url: 'https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=2694&auto=format&fit=crop', name: 'Neon Mountains' },
    { url: 'https://images.unsplash.com/photo-1604871000636-074fa5117945?q=80&w=2574&auto=format&fit=crop', name: 'Dark Mesh' },
    { url: 'https://images.unsplash.com/photo-1557672172-298e090bd0f1?q=80&w=2574&auto=format&fit=crop', name: 'Gradient Flow' },
    { url: 'https://images.unsplash.com/photo-1518770660439-4636190af475?q=80&w=2670&auto=format&fit=crop', name: 'Circuit Board' }
  ]

  let daemonStatus = 'loading'
  let hardware = null
  let projects = []
  let portsSnapshot = null
  let portsStatus = 'idle'
  let portsExpanded = false
  let currentWallpaper = WALLPAPERS[0].url
  let showWallpaperPicker = false
  let sysinfoInterval
  let projectsInterval
  let portsInterval

  function loadWallpaper() {
    try {
      const saved = localStorage.getItem('dashWallpaper')
      if (saved) currentWallpaper = saved
    } catch {}
  }

  function saveWallpaper(url) {
    currentWallpaper = url
    try { localStorage.setItem('dashWallpaper', url) } catch {}
  }

  function handleWallpaperSelect(url) {
    saveWallpaper(url)
    showWallpaperPicker = false
  }

  function shuffleWallpaper() {
    let next
    do { next = WALLPAPERS[Math.floor(Math.random() * WALLPAPERS.length)].url }
    while (next === currentWallpaper && WALLPAPERS.length > 1)
    saveWallpaper(next)
  }

  async function pollSysinfo() {
    try {
      hardware = await fetchSysinfo()
      daemonStatus = 'online'
    } catch (error) {
      console.warn('Sysinfo poll failed:', error.message)
      daemonStatus = 'offline'
    }
  }

  async function pollProjects() {
    try {
      projects = await fetchProjects()
    } catch (error) {
      console.warn('Projects poll failed:', error.message)
    }
  }

  async function pollPorts() {
    try {
      if (!portsSnapshot) portsStatus = 'loading'
      portsSnapshot = await fetchPorts()
      portsStatus = 'online'
    } catch (error) {
      console.warn('Ports poll failed:', error.message)
      portsStatus = 'offline'
    }
  }

  function startPortsPolling() {
    if (portsInterval) return
    pollPorts()
    portsInterval = setInterval(pollPorts, 10000)
  }

  function stopPortsPolling() {
    clearInterval(portsInterval)
    portsInterval = null
  }

  function togglePorts() {
    portsExpanded = !portsExpanded
    if (portsExpanded) startPortsPolling()
    else stopPortsPolling()
  }

  onMount(() => {
    loadWallpaper()
    pollSysinfo()
    pollProjects()
    sysinfoInterval = setInterval(pollSysinfo, 2000)
    projectsInterval = setInterval(pollProjects, 5000)
  })

  onDestroy(() => {
    clearInterval(sysinfoInterval)
    clearInterval(projectsInterval)
    clearInterval(portsInterval)
  })
</script>

<Hero
  {daemonStatus}
  {currentWallpaper}
  onWallpaperClick={() => (showWallpaperPicker = true)}
  onShuffleClick={shuffleWallpaper}
/>

<main class="dashboard">
  <div class="dashboard-split">
    <div class="telemetry-column">
      <HardwarePanel {hardware} status={daemonStatus} />
      <StoragePanel storage={hardware?.storage || []} status={daemonStatus} />
    </div>

    <aside class="activity-rail" aria-label="Bookmarks and project activity">
      <BookmarksPanel />
      <ProjectsPanel {projects} compact={true} onRefresh={pollProjects} />
    </aside>
  </div>

  <section class="port-audit">
    <button
      class="port-toggle"
      type="button"
      aria-expanded={portsExpanded}
      aria-controls="port-audit-panel"
      on:click={togglePorts}
    >
      <span class="port-toggle-copy">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10Z"></path>
          <path d="M9 12h6"></path>
        </svg>
        <span>
          <strong>Port audit</strong>
          <small>{portsExpanded ? 'Listening services are shown below' : 'Inspect local listening services'}</small>
        </span>
      </span>
      <svg class="chevron" class:expanded={portsExpanded} viewBox="0 0 24 24" aria-hidden="true">
        <path d="m6 9 6 6 6-6"></path>
      </svg>
    </button>

    {#if portsExpanded}
      <div id="port-audit-panel">
        <PortsPanel snapshot={portsSnapshot} status={portsStatus} onRefresh={pollPorts} />
      </div>
    {/if}
  </section>
</main>

{#if showWallpaperPicker}
  <WallpaperPicker
    wallpapers={WALLPAPERS}
    {currentWallpaper}
    onSelect={handleWallpaperSelect}
    onClose={() => (showWallpaperPicker = false)}
  />
{/if}

<style>
  .dashboard {
    width: 100%;
    padding: clamp(var(--sp-5), 3vw, var(--sp-10));
    background: var(--bg);
  }

  .dashboard-split {
    display: grid;
    grid-template-columns: minmax(0, 13fr) minmax(21rem, 7fr);
    gap: clamp(var(--sp-5), 2.5vw, var(--sp-10));
    align-items: start;
  }

  .telemetry-column {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: var(--sp-8);
  }

  .activity-rail {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: 0.5rem;
    overflow: hidden;
    border: 0.0625rem solid var(--line);
    border-radius: var(--radius-lg);
    background: var(--accent-strong);
  }

  .activity-rail :global(.card) {
    border: 0;
    border-radius: 0;
  }

  .port-audit {
    margin-top: var(--sp-12);
  }

  .port-toggle {
    display: flex;
    width: 100%;
    min-height: 4.5rem;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-4);
    padding: var(--sp-3) var(--sp-5);
    border: 0.0625rem solid var(--line);
    border-radius: var(--radius-lg);
    background: var(--surface);
    color: var(--text-secondary);
    text-align: left;
    transition: background-color 180ms ease, border-color 180ms ease;
  }

  .port-toggle:hover {
    border-color: var(--line-strong);
    background: var(--surface-raised);
    color: var(--text-primary);
  }

  .port-toggle-copy {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
  }

  .port-toggle-copy > svg {
    width: 1.5rem;
    fill: none;
    stroke: var(--accent);
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 1.8;
  }

  .port-toggle-copy > span {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
  }

  .port-toggle strong {
    color: var(--text-primary);
    font-size: 1rem;
    font-weight: 650;
  }

  .port-toggle small {
    font-size: 0.875rem;
  }

  .chevron {
    width: 1.25rem;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 2;
    transition: transform 180ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .chevron.expanded {
    transform: rotate(180deg);
  }

  #port-audit-panel {
    margin-top: var(--sp-3);
  }

  @media (max-width: 68.75rem) {
    .dashboard-split {
      grid-template-columns: 1fr;
    }

    .activity-rail {
      display: grid;
      grid-template-columns: minmax(0, 1.2fr) minmax(18rem, 0.8fr);
    }
  }

  @media (max-width: 47.5rem) {
    .dashboard {
      padding: var(--sp-4);
    }

    .activity-rail {
      display: flex;
      flex-direction: column;
    }

    .port-audit {
      margin-top: var(--sp-8);
    }
  }
</style>
