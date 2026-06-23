<script>
  import { onMount } from 'svelte';

  // Pinned / Favorite Sites (LocalStorage)
  let pinnedSites = [];
  let newSiteName = '';
  let newSiteUrl = '';
  let showAddForm = false;

  // Custom Bookmarks (chrome.storage.local)
  let customBookmarks = [];
  let bookmarkSearchQuery = '';
  let filteredBookmarks = [];
  let newBookmarkName = '';
  let newBookmarkUrl = '';
  let showAddBookmarkForm = false;

  const DEFAULT_PINNED = [
    { name: 'Google', url: 'https://google.com', icon: 'fa-google', color: '#4285f4' },
    { name: 'YouTube', url: 'https://youtube.com', icon: 'fa-youtube-play', color: '#ff0000' },
    { name: 'GitHub', url: 'https://github.com', icon: 'fa-github', color: '#a78bfa' },
    { name: 'Reddit', url: 'https://reddit.com', icon: 'fa-reddit', color: '#ff4500' }
  ];

  function loadPinned() {
    try {
      const stored = localStorage.getItem('dashPinnedSites');
      if (stored) {
        pinnedSites = JSON.parse(stored);
      } else {
        pinnedSites = DEFAULT_PINNED;
        savePinned();
      }
    } catch (e) {
      pinnedSites = DEFAULT_PINNED;
    }
  }

  function savePinned() {
    localStorage.setItem('dashPinnedSites', JSON.stringify(pinnedSites));
  }

  function addPinned() {
    if (!newSiteName.trim() || !newSiteUrl.trim()) return;
    let url = newSiteUrl.trim();
    if (!/^https?:\/\//i.test(url)) {
      url = 'https://' + url;
    }

    const icons = ['fa-globe', 'fa-link', 'fa-external-link', 'fa-laptop'];
    const colors = ['#6366f1', '#ec4899', '#14b8a6', '#f59e0b', '#8b5cf6'];
    
    pinnedSites = [...pinnedSites, {
      name: newSiteName.trim(),
      url,
      icon: icons[Math.floor(Math.random() * icons.length)],
      color: colors[Math.floor(Math.random() * colors.length)]
    }];
    savePinned();

    newSiteName = '';
    newSiteUrl = '';
    showAddForm = false;
  }

  function removePinned(index) {
    pinnedSites = pinnedSites.filter((_, i) => i !== index);
    savePinned();
  }

  // Load Custom Bookmarks
  function loadCustomBookmarks() {
    if (typeof chrome !== 'undefined' && chrome.storage && chrome.storage.local) {
      chrome.storage.local.get(['dashCustomBookmarks'], (result) => {
        if (result.dashCustomBookmarks) {
          customBookmarks = result.dashCustomBookmarks;
        } else {
          customBookmarks = [
            { name: 'Svelte Docs', url: 'https://svelte.dev' },
            { name: 'Vite Guide', url: 'https://vite.dev' }
          ];
          saveCustomBookmarks();
        }
        filterBookmarks();
      });
    } else {
      // Fallback for dev environment
      const stored = localStorage.getItem('dashCustomBookmarks');
      if (stored) {
        customBookmarks = JSON.parse(stored);
      } else {
        customBookmarks = [
          { name: 'Svelte Docs', url: 'https://svelte.dev' },
          { name: 'Vite Guide', url: 'https://vite.dev' }
        ];
        saveCustomBookmarks();
      }
      filterBookmarks();
    }
  }

  function saveCustomBookmarks() {
    if (typeof chrome !== 'undefined' && chrome.storage && chrome.storage.local) {
      chrome.storage.local.set({ dashCustomBookmarks: customBookmarks });
    } else {
      localStorage.setItem('dashCustomBookmarks', JSON.stringify(customBookmarks));
    }
    filterBookmarks();
  }

  function addCustomBookmark() {
    if (!newBookmarkName.trim() || !newBookmarkUrl.trim()) return;
    let url = newBookmarkUrl.trim();
    if (!/^https?:\/\//i.test(url)) {
      url = 'https://' + url;
    }
    customBookmarks = [...customBookmarks, { name: newBookmarkName.trim(), url }];
    saveCustomBookmarks();
    newBookmarkName = '';
    newBookmarkUrl = '';
    showAddBookmarkForm = false;
  }

  function removeCustomBookmark(index) {
    // We need to find the actual index in customBookmarks, not the filtered array
    const itemToRemove = filteredBookmarks[index];
    customBookmarks = customBookmarks.filter(b => b !== itemToRemove);
    saveCustomBookmarks();
  }

  function filterBookmarks() {
    if (!bookmarkSearchQuery.trim()) {
      filteredBookmarks = customBookmarks.slice(0, 15); // Show first 15 default
    } else {
      const q = bookmarkSearchQuery.toLowerCase();
      filteredBookmarks = customBookmarks.filter(b => 
        b.name.toLowerCase().includes(q) || b.url.toLowerCase().includes(q)
      ).slice(0, 15);
    }
  }

  $: {
    bookmarkSearchQuery;
    if (customBookmarks.length > 0) {
      filterBookmarks();
    }
  }

  onMount(() => {
    loadPinned();
    loadCustomBookmarks();
  });
</script>

<div class="grid grid-cols-1 lg:grid-cols-5 gap-6 mt-6 select-none">
  
  <!-- Left Side: Pinned Sites (Quick Access) -->
  <div class="lg:col-span-3 glass rounded-2xl p-5 text-left flex flex-col justify-between hover:border-slate-700/50 transition-all duration-300">
    <div>
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-indigo-500/10 flex items-center justify-center">
            <i class="fa fa-star text-indigo-400 text-lg"></i>
          </div>
          <div>
            <h2 class="text-base font-semibold text-white">Pinned Shortcuts</h2>
            <p class="text-xs text-slate-500">Quick launch favorite links</p>
          </div>
        </div>
        <button on:click={() => showAddForm = !showAddForm} 
                class="text-xs text-indigo-400 hover:text-indigo-300 transition flex items-center gap-1 font-semibold"
        >
          <i class="fa fa-plus-circle text-sm"></i> Add Pin
        </button>
      </div>

      <!-- Add Pin Form -->
      {#if showAddForm}
        <div class="mb-4 bg-slate-900/50 border border-white/5 rounded-xl p-4 flex flex-col sm:flex-row gap-3">
          <input type="text" bind:value={newSiteName} placeholder="Name (e.g. GitHub)" 
                 class="flex-1 bg-white/5 border border-white/5 rounded-xl px-4 py-2.5 text-xs text-white placeholder-slate-500 focus:border-indigo-500/50 outline-none transition"
          />
          <input type="text" bind:value={newSiteUrl} placeholder="URL (e.g. github.com)" 
                 class="flex-1 bg-white/5 border border-white/5 rounded-xl px-4 py-2.5 text-xs text-white placeholder-slate-500 focus:border-indigo-500/50 outline-none transition"
          />
          <button on:click={addPinned} 
                  class="bg-indigo-500 hover:bg-indigo-600 text-slate-950 font-bold text-xs rounded-xl px-4 py-2.5 transition flex items-center justify-center gap-1"
          >
            Add
          </button>
        </div>
      {/if}

      <!-- Grid of Pinned Sites -->
      <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 gap-3">
        {#each pinnedSites as site, i}
          <a href={site.url} target="_blank" rel="noopener noreferrer"
             class="group relative flex flex-col items-center gap-2 p-3 bg-slate-950/20 hover:bg-slate-900/50 border border-white/5 hover:border-slate-800 rounded-2xl transition-all duration-300"
          >
            <!-- Delete Button -->
            <button on:click|preventDefault|stopPropagation={() => removePinned(i)}
                    class="absolute top-1.5 right-1.5 w-4 h-4 rounded-full bg-rose-500/10 text-rose-400 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center hover:bg-rose-500/20"
                    title="Remove pin"
            >
              <i class="fa fa-times text-[10px]"></i>
            </button>

            <!-- Icon -->
            <div class="w-11 h-11 rounded-xl flex items-center justify-center text-xl transition-transform duration-300 group-hover:scale-105"
                 style="background: {site.color}15; color: {site.color}"
            >
              <i class="fa {site.icon}"></i>
            </div>
            <span class="text-xs text-slate-300 group-hover:text-white truncate w-full text-center">
              {site.name}
            </span>
          </a>
        {/each}
      </div>
    </div>
  </div>

  <!-- Right Side: Browser Bookmarks Search -->
  <div class="lg:col-span-2 glass rounded-2xl p-5 text-left flex flex-col hover:border-slate-700/50 transition-all duration-300">
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-cyan-500/10 flex items-center justify-center">
          <i class="fa fa-bookmark text-cyan-400 text-lg"></i>
        </div>
        <div>
          <h2 class="text-base font-semibold text-white">Custom Bookmarks</h2>
          <p class="text-xs text-slate-500">Your saved links</p>
        </div>
      </div>
      <button on:click={() => showAddBookmarkForm = !showAddBookmarkForm} 
              class="text-xs text-cyan-400 hover:text-cyan-300 transition flex items-center gap-1 font-semibold"
      >
        <i class="fa fa-plus-circle text-sm"></i> Add Link
      </button>
    </div>

    <!-- Add Bookmark Form -->
    {#if showAddBookmarkForm}
      <div class="mb-3 bg-slate-900/50 border border-white/5 rounded-xl p-3 flex flex-col gap-2">
        <input type="text" bind:value={newBookmarkName} placeholder="Name" 
               class="w-full bg-white/5 border border-white/5 rounded-lg px-3 py-2 text-xs text-white placeholder-slate-500 focus:border-cyan-500/50 outline-none transition"
        />
        <div class="flex gap-2">
          <input type="text" bind:value={newBookmarkUrl} placeholder="URL" 
                 class="flex-1 bg-white/5 border border-white/5 rounded-lg px-3 py-2 text-xs text-white placeholder-slate-500 focus:border-cyan-500/50 outline-none transition"
          />
          <button on:click={addCustomBookmark} 
                  class="bg-cyan-500 hover:bg-cyan-600 text-slate-950 font-bold text-xs rounded-lg px-3 py-2 transition"
          >
            Add
          </button>
        </div>
      </div>
    {/if}

    <!-- Search Input -->
    <div class="relative mb-3">
      <input type="text" bind:value={bookmarkSearchQuery} placeholder="Search bookmarks..." 
             class="w-full bg-white/5 border border-white/5 rounded-xl pl-9 pr-4 py-2.5 text-xs text-white placeholder-slate-500 focus:border-cyan-500/50 outline-none transition"
      />
      <i class="fa fa-search absolute left-3 top-3 text-slate-500 text-xs"></i>
    </div>

    <!-- Bookmarks List -->
    <div class="space-y-1.5 overflow-y-auto max-h-[160px] pr-1 scrollbar-thin">
      {#each filteredBookmarks as bookmark, index}
        <div class="flex items-center justify-between p-2 hover:bg-slate-900/50 border border-transparent hover:border-white/5 rounded-xl transition text-xs group relative">
          <a href={bookmark.url} target="_blank" rel="noopener noreferrer" class="flex items-center justify-between flex-1 truncate pr-6">
            <span class="text-slate-300 group-hover:text-white truncate font-medium max-w-[150px]" title={bookmark.name}>
              {bookmark.name || 'Untitled'}
            </span>
            <span class="text-[10px] text-slate-500 truncate max-w-[120px] font-mono group-hover:text-cyan-400/80">
              {bookmark.url.replace(/^https?:\/\/(www\.)?/, '')}
            </span>
          </a>
          <button on:click={() => removeCustomBookmark(index)}
                  class="absolute right-2 opacity-0 group-hover:opacity-100 transition-opacity text-rose-400 hover:text-rose-300 w-5 h-5 flex items-center justify-center rounded bg-rose-500/10 hover:bg-rose-500/20"
                  title="Remove bookmark"
          >
            <i class="fa fa-trash text-[10px]"></i>
          </button>
        </div>
      {:else}
        <p class="text-xs text-slate-500 text-center py-4">No matching bookmarks found.</p>
      {/each}
    </div>
  </div>
</div>
