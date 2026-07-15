<script>
  import { onMount, onDestroy } from 'svelte'
  import { fetchSysinfo, fetchProjects, fetchPorts } from './lib/api.js'

  import Hero from './components/Hero.svelte'
  import Sidebar from './components/Sidebar.svelte'
  import HardwarePanel from './components/HardwarePanel.svelte'
  import StoragePanel from './components/StoragePanel.svelte'
  import BookmarksPanel from './components/BookmarksPanel.svelte'
  import ProjectsPanel from './components/ProjectsPanel.svelte'
  import PortsPanel from './components/PortsPanel.svelte'
  import WallpaperPicker from './components/WallpaperPicker.svelte'

  // Default wallpapers (these must exist in public/ or be valid remote URLs)
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
  let activeView = 'hardware'

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
      const data = await fetchSysinfo()
      hardware = data
      daemonStatus = 'online'
    } catch (e) {
      console.warn('Sysinfo poll failed:', e.message)
      daemonStatus = 'offline'
    }
  }

  async function pollProjects() {
    try {
      const data = await fetchProjects()
      projects = data
    } catch (e) {
      console.warn('Projects poll failed:', e.message)
    }
  }

  async function pollPorts() {
    try {
      if (!portsSnapshot) portsStatus = 'loading'
      portsSnapshot = await fetchPorts()
      portsStatus = 'online'
    } catch (e) {
      console.warn('Ports poll failed:', e.message)
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

  function setActiveView(view) {
    activeView = view
    if (view === 'ports') startPortsPolling()
    else stopPortsPolling()
  }

  onMount(() => {
    loadWallpaper()
    
    // Initial fetches
    pollSysinfo()
    pollProjects()

    // Setup polling loops
    sysinfoInterval = setInterval(pollSysinfo, 3000)
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

<div class="main-layout">
  <Sidebar {activeView} onViewChange={setActiveView} />
  
  <main class="content-area">
    {#if activeView === 'hardware'}
      <div class="hardware-view">
        <div class="hw-col-main">
          <HardwarePanel {hardware} status={daemonStatus} />
          <StoragePanel storage={hardware?.storage || []} status={daemonStatus} />
        </div>
        <div class="hw-col-side">
          <BookmarksPanel compact={true} />
          <ProjectsPanel {projects} compact={true} onRefresh={pollProjects} />
        </div>
      </div>
    {:else if activeView === 'bookmarks'}
      <div class="bookmarks-view">
        <BookmarksPanel compact={false} />
      </div>
    {:else if activeView === 'ports'}
      <div class="ports-view">
        <PortsPanel snapshot={portsSnapshot} status={portsStatus} onRefresh={pollPorts} />
      </div>
    {/if}
  </main>
</div>

{#if showWallpaperPicker}
  <WallpaperPicker
    wallpapers={WALLPAPERS}
    {currentWallpaper}
    onSelect={handleWallpaperSelect}
    onClose={() => (showWallpaperPicker = false)}
  />
{/if}

<style>
  .main-layout {
    display: grid;
    grid-template-columns: 180px 1fr;
    min-height: calc(100vh - 320px);
    background: var(--bg);
  }

  .content-area {
    padding: var(--sp-6) var(--sp-8);
  }

  .hardware-view {
    display: grid;
    grid-template-columns: 1fr 380px;
    gap: var(--sp-6);
    align-items: start;
  }

  .hw-col-main, .hw-col-side {
    display: flex;
    flex-direction: column;
    gap: var(--sp-6);
  }

  .bookmarks-view {
    max-width: 900px;
    margin: 0 auto;
  }

  .ports-view {
    max-width: 1100px;
    margin: 0 auto;
  }

  @media (max-width: 1200px) {
    .hardware-view {
      grid-template-columns: 1fr 320px;
    }
  }

  @media (max-width: 900px) {
    .main-layout {
      grid-template-columns: 1fr;
    }
    .hardware-view {
      grid-template-columns: 1fr;
    }
    .content-area {
      padding: var(--sp-4);
    }
  }
</style>
