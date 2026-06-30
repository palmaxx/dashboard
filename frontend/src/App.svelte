<script>
  import { onMount } from 'svelte';
  import Clock from './components/Clock.svelte';
  import Stats from './components/Stats.svelte';
  import Bookmarks from './components/Bookmarks.svelte';
  import Todos from './components/Todos.svelte';

  const wallpapers = [
    {
      url: './wallpapers/command-mountain.png',
      name: 'Command Mountain',
    },
  ];

  let currentWallpaper = wallpapers[0].url;
  let showWallpaperPicker = false;
  let customWallpaperUrl = '';
  let activeView = 'hardware';

  let hardware = null;
  let status = 'loading';
  let lastUpdated = null;
  let showStatusBar = true;
  let hideStatusTimeout;

  function storeWallpaper(url) {
    try {
      localStorage.setItem('dashWallpaper', url);
    } catch (error) {
      console.warn('Unable to persist wallpaper', error);
    }
  }

  function setWallpaper(url) {
    currentWallpaper = url;
    storeWallpaper(url);
    showWallpaperPicker = false;
  }

  function randomWallpaper() {
    const remaining = wallpapers.filter((wallpaper) => wallpaper.url !== currentWallpaper);
    const chosen = remaining[Math.floor(Math.random() * remaining.length)] || wallpapers[0];
    setWallpaper(chosen.url);
  }

  function handleCustomUrlApply() {
    const trimmed = customWallpaperUrl.trim();
    if (!trimmed) return;
    setWallpaper(trimmed);
    customWallpaperUrl = '';
  }

  function handleFileUpload(event) {
    const file = event.target.files?.[0];
    if (!file) return;

    if (file.size > 2 * 1024 * 1024) {
      alert('Image is too large. Please choose an image under 2MB.');
      return;
    }

    const reader = new FileReader();
    reader.onload = (loadEvent) => {
      if (loadEvent.target?.result) {
        setWallpaper(loadEvent.target.result);
      }
    };
    reader.readAsDataURL(file);
  }

  function flashStatusBar() {
    showStatusBar = true;
    if (hideStatusTimeout) {
      clearTimeout(hideStatusTimeout);
    }
    hideStatusTimeout = setTimeout(() => {
      if (status === 'online') {
        showStatusBar = false;
      }
    }, 4200);
  }

  async function fetchSystemInfo() {
    try {
      const response = await fetch('http://127.0.0.1:9999/api/sysinfo', { cache: 'no-store' });
      if (!response.ok) {
        throw new Error(`Daemon returned ${response.status}`);
      }

      hardware = await response.json();
      lastUpdated = new Date();
      if (status !== 'online') {
        status = 'online';
        flashStatusBar();
      } else {
        status = 'online';
      }
    } catch (error) {
      if (status !== 'offline') {
        status = 'offline';
        flashStatusBar();
      } else {
        status = 'offline';
      }
    }
  }

  function handleGlobalKeydown(event) {
    if (event.key === 'Escape' && showWallpaperPicker) {
      showWallpaperPicker = false;
    }
  }

  onMount(() => {
    try {
      const storedWallpaper = localStorage.getItem('dashWallpaper');
      if (storedWallpaper) {
        currentWallpaper = storedWallpaper;
      }
    } catch (error) {
      console.warn('Unable to read saved wallpaper', error);
    }

    fetchSystemInfo();

    const hardwareInterval = setInterval(fetchSystemInfo, 3000);

    return () => {
      clearInterval(hardwareInterval);
      if (hideStatusTimeout) {
        clearTimeout(hideStatusTimeout);
      }
    };
  });
</script>

<svelte:window on:keydown={handleGlobalKeydown} />

<main class="deck-app">
  <section class="hero-shell" style={`background-image: linear-gradient(180deg, rgba(6, 10, 14, 0.16), rgba(6, 10, 14, 0.68)), url("${currentWallpaper}")`}>
    <div class="hero-topline">
      <a href="./index.html" class="brand-mark" aria-label="Sleek Dashboard home">
        <span class="brand-glyph">SD</span>
        <span>Sleek Dashboard</span>
      </a>

      <div class="hero-actions">
        <button type="button" class="icon-action wide" on:click={() => (showWallpaperPicker = true)}>
          <i class="fa fa-picture-o" aria-hidden="true"></i>
          <span>Wallpaper</span>
        </button>
        <button type="button" class="icon-action" aria-label="Shuffle wallpaper" title="Shuffle wallpaper" on:click={randomWallpaper}>
          <i class="fa fa-random" aria-hidden="true"></i>
        </button>
      </div>
    </div>

    <div class="hero-center">
      <Clock connectionStatus={status} />
    </div>

    <div class="daemon-pill" class:online={status === 'online'} class:offline={status === 'offline'}>
      <span></span>
      {#if status === 'online'}
        Local Daemon Connected
      {:else if status === 'offline'}
        Local Daemon Offline
      {:else}
        Connecting to Local Daemon
      {/if}
    </div>
  </section>

  <section class="deck-workspace">
    <nav class="command-rail" aria-label="Dashboard sections">
      <button
        type="button"
        class:active={activeView === 'hardware'}
        aria-pressed={activeView === 'hardware'}
        on:click={() => (activeView = 'hardware')}
      >
        <i class="fa fa-tachometer" aria-hidden="true"></i>
        <span>Hardware</span>
      </button>
      <button
        type="button"
        class:active={activeView === 'bookmarks'}
        aria-pressed={activeView === 'bookmarks'}
        on:click={() => (activeView = 'bookmarks')}
      >
        <i class="fa fa-bookmark-o" aria-hidden="true"></i>
        <span>Bookmarks</span>
      </button>
      <div class="rail-spacer"></div>
    </nav>

    <div class="workspace-grid" class:bookmark-focus={activeView === 'bookmarks'}>
      {#if activeView === 'hardware'}
        <section class="primary-stack" aria-label="Hardware dashboard">
          <Stats {hardware} {status} {lastUpdated} {showStatusBar} />
        </section>
        <aside class="side-stack" aria-label="Quick access and projects">
          <Bookmarks variant="compact" />
          <Todos variant="compact" />
        </aside>
      {:else}
        <section class="primary-stack" aria-label="Bookmarks dashboard">
          <Bookmarks variant="full" />
        </section>
        <aside class="side-stack" aria-label="System and projects summary">
          <Stats {hardware} {status} {lastUpdated} {showStatusBar} compact />
          <Todos variant="compact" />
        </aside>
      {/if}
    </div>
  </section>

  {#if showWallpaperPicker}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-backdrop" on:click|self={() => (showWallpaperPicker = false)}>
      <section class="wallpaper-modal" aria-label="Choose wallpaper">
        <header class="modal-header">
          <div>
            <p>Visual System</p>
            <h2>Choose Background</h2>
          </div>
          <button type="button" class="icon-action" aria-label="Close wallpaper picker" on:click={() => (showWallpaperPicker = false)}>
            <i class="fa fa-times" aria-hidden="true"></i>
          </button>
        </header>

        <div class="wallpaper-grid">
          {#each wallpapers as wallpaper}
            <button
              type="button"
              class:active={currentWallpaper === wallpaper.url}
              on:click={() => setWallpaper(wallpaper.url)}
            >
              <img src={wallpaper.url} alt={wallpaper.name} />
              <span>{wallpaper.name}</span>
            </button>
          {/each}
        </div>

        <div class="custom-wallpaper">
          <label>
            <span>Upload Image</span>
            <input type="file" accept="image/*" on:change={handleFileUpload} />
          </label>
          <div class="url-entry">
            <input
              type="text"
              bind:value={customWallpaperUrl}
              aria-label="Image URL"
              on:keydown={(event) => event.key === 'Enter' && handleCustomUrlApply()}
            />
            <button type="button" on:click={handleCustomUrlApply}>Apply</button>
          </div>
          <p>Uploads are stored in browser storage. Keep files under 2MB.</p>
        </div>
      </section>
    </div>
  {/if}
</main>
