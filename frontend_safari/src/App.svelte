<script>
  import Clock from './components/Clock.svelte';
  import Stats from './components/Stats.svelte';
  import Bookmarks from './components/Bookmarks.svelte';
  import Todos from './components/Todos.svelte';
  import { onMount } from 'svelte';

  const wallpapers = [
    { url: 'https://images.unsplash.com/photo-1506744038136-46273834b3fb?w=1920&q=80', name: 'Yosemite' },
    { url: 'https://images.unsplash.com/photo-1470071459604-3b5ec3a7fe05?w=1920&q=80', name: 'Forest Mist' },
    { url: 'https://images.unsplash.com/photo-1519681393784-d120267933ba?w=1920&q=80', name: 'Starry Peaks' },
    { url: 'https://images.unsplash.com/photo-1501785888041-af3ef285b470?w=1920&q=80', name: 'Lake Sunset' },
    { url: 'https://images.unsplash.com/photo-1433086966358-54859d0ed716?w=1920&q=80', name: 'Mountain River' }
  ];

  let currentWallpaper = wallpapers[0].url;
  let showWallpaperPicker = false;

  // Weather simulation
  let temp = 22;
  let condition = 'Partly Cloudy';
  
  function updateWeather() {
    const temps = [18, 20, 22, 23, 25, 21, 19];
    const conds = ['Sunny', 'Partly Cloudy', 'Clear Sky', 'Overcast', 'Light Breeze'];
    temp = temps[Math.floor(Math.random() * temps.length)];
    condition = conds[Math.floor(Math.random() * conds.length)];
  }

  function setWallpaper(url) {
    currentWallpaper = url;
    localStorage.setItem('dashWallpaper', url);
    showWallpaperPicker = false;
  }

  function randomWallpaper() {
    const remaining = wallpapers.filter(w => w.url !== currentWallpaper);
    const chosen = remaining[Math.floor(Math.random() * remaining.length)] || wallpapers[0];
    setWallpaper(chosen.url);
  }

  let customWallpaperUrl = '';

  function handleCustomUrlApply() {
    if (customWallpaperUrl.trim()) {
      setWallpaper(customWallpaperUrl.trim());
      customWallpaperUrl = '';
    }
  }

  function handleFileUpload(event) {
    const file = event.target.files[0];
    if (!file) return;

    if (file.size > 2 * 1024 * 1024) {
      alert("Image is too large! Please choose an image under 2MB to fit in browser storage.");
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      setWallpaper(e.target.result);
    };
    reader.readAsDataURL(file);
  }

  // System metrics state and polling
  let hardware = null;
  let status = 'loading'; // 'loading' | 'online' | 'offline'
  let lastUpdated = null;
  let showStatusBar = true;
  let hideStatusTimeout = null;

  async function fetchSystemInfo() {
    try {
      const res = await fetch('http://127.0.0.1:9999/api/sysinfo', { cache: 'no-store' });
      if (res.ok) {
        const data = await res.json();
        hardware = data;
        lastUpdated = new Date();
        
        if (status !== 'online') {
          status = 'online';
          showStatusBar = true;
          if (hideStatusTimeout) clearTimeout(hideStatusTimeout);
          hideStatusTimeout = setTimeout(() => {
            showStatusBar = false;
          }, 5000);
        }
      } else {
        handleOffline();
      }
    } catch (err) {
      handleOffline();
    }
  }

  function handleOffline() {
    if (status !== 'offline') {
      status = 'offline';
      showStatusBar = true;
      if (hideStatusTimeout) {
        clearTimeout(hideStatusTimeout);
        hideStatusTimeout = null;
      }
    }
  }

  onMount(() => {
    const saved = localStorage.getItem('dashWallpaper');
    if (saved) {
      currentWallpaper = saved;
    }
    updateWeather();

    // Initial query
    fetchSystemInfo();
    // Query every 3 seconds
    const intervalId = setInterval(fetchSystemInfo, 3000);

    return () => {
      clearInterval(intervalId);
      if (hideStatusTimeout) clearTimeout(hideStatusTimeout);
    };
  });
</script>

<main class="min-h-screen text-slate-200 bg-[#0a0a0f] relative font-sans pb-10">
  
  <!-- Hero Section with Background Wallpaper -->
  <div class="relative w-full h-[52vh] min-h-[340px] bg-cover bg-center transition-all duration-700"
       style="background-image: url('{currentWallpaper}')"
  >
    <!-- Dark overlay with smooth gradient fade to the background around the boxes -->
    <div class="absolute inset-0 bg-gradient-to-b from-black/50 via-black/15 via-[#0a0a0f]/85 to-[#0a0a0f]"></div>
    
    <!-- Time, Date, and Greeting (shifted up to prevent collision with overlapping cards) -->
    <div class="absolute inset-0 flex flex-col items-center justify-center pb-32 z-10 px-4">
      <Clock connectionStatus={status} />
      
      <!-- Weather Badge -->
      <div class="flex items-center gap-4 text-xs font-semibold text-slate-300 mt-2 bg-black/30 backdrop-blur-md px-3.5 py-1.5 rounded-full border border-white/5 select-none">
        <span><i class="fa fa-thermometer-half mr-1.5 text-indigo-400"></i> {temp}°C</span>
        <span class="w-1.5 h-1.5 bg-slate-600 rounded-full"></span>
        <span><i class="fa fa-cloud mr-1.5 text-sky-400"></i> {condition}</span>
      </div>
    </div>
    
    <!-- Wallpaper Picker Controls (Moved to top-right to avoid collisions) -->
    <div class="absolute top-4 right-4 z-20 flex items-center gap-2">
      <button on:click={() => showWallpaperPicker = true} 
              class="glass px-3.5 py-2 rounded-xl text-xs font-semibold text-slate-300 hover:bg-white/10 hover:text-white transition flex items-center gap-2 select-none"
      >
        <i class="fa fa-picture-o"></i> Wallpaper
      </button>
      <button on:click={randomWallpaper} 
              class="glass px-3.5 py-2 rounded-xl text-xs text-slate-300 hover:bg-white/10 hover:text-white transition select-none"
              title="Shuffle wallpaper"
      >
        <i class="fa fa-refresh"></i>
      </button>
    </div>
  </div>

  <!-- Wallpaper Picker Modal -->
  {#if showWallpaperPicker}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm"
         on:click|self={() => showWallpaperPicker = false}
    >
      <div class="glass rounded-2xl p-6 max-w-xl w-full mx-4 max-h-[85vh] overflow-y-auto border border-white/10">
        <div class="flex items-center justify-between mb-4 select-none">
          <h3 class="text-base font-bold text-white">Choose Background</h3>
          <button on:click={() => showWallpaperPicker = false} 
                  class="text-slate-400 hover:text-white transition"
          >
            <i class="fa fa-times text-lg"></i>
          </button>
        </div>
        <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
          {#each wallpapers as w}
            <button on:click={() => setWallpaper(w.url)} 
                    class="group relative aspect-video rounded-xl overflow-hidden border-2 transition-all duration-300
                           {currentWallpaper === w.url ? 'border-indigo-500' : 'border-transparent hover:border-slate-500'}"
            >
              <img src={w.url} alt={w.name} class="w-full h-full object-cover group-hover:scale-105 transition duration-500" />
              <div class="absolute inset-0 bg-gradient-to-t from-black/70 to-transparent flex items-end p-2 select-none">
                <span class="text-[10px] font-bold text-white truncate">{w.name}</span>
              </div>
            </button>
          {/each}
        </div>

        <!-- Custom Wallpaper Section -->
        <div class="mt-6 border-t border-white/5 pt-4 text-left">
          <h4 class="text-xs font-semibold uppercase tracking-wider text-slate-400 mb-2">Use Custom Background</h4>
          <div class="flex flex-col gap-3">
            <!-- File Upload -->
            <div class="flex items-center gap-2">
              <label class="glass px-3.5 py-2 rounded-xl text-xs text-slate-300 hover:bg-white/10 hover:text-white cursor-pointer select-none">
                <i class="fa fa-upload mr-1.5"></i> Upload Image
                <input type="file" accept="image/*" on:change={handleFileUpload} class="hidden" />
              </label>
              <span class="text-[10px] text-slate-500">Max size: 2MB</span>
            </div>

            <!-- URL Input -->
            <div class="flex gap-2">
              <input type="text" bind:value={customWallpaperUrl} placeholder="Paste image URL (e.g. from Unsplash)..."
                     class="flex-1 bg-white/5 border border-white/5 rounded-xl px-4 py-2 text-xs text-white placeholder-slate-500 focus:border-indigo-500/50 outline-none transition"
              />
              <button on:click={handleCustomUrlApply}
                      class="bg-indigo-500 hover:bg-indigo-600 text-slate-950 font-bold text-xs rounded-xl px-4 py-2 transition"
              >
                Apply
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Main Content Dashboard Container (Pulled higher on faded wallpaper background) -->
  <div class="max-w-7xl mx-auto px-4 pb-12 -mt-20 relative z-20">
    <Stats {hardware} {status} {lastUpdated} {showStatusBar} />
    <Bookmarks />
    <Todos />
  </div>

</main>
