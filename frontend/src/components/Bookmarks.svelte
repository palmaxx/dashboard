<script>
  import { onMount } from 'svelte';

  // Pinned / Favorite Sites (LocalStorage)
  let pinnedSites = [];
  let newSiteName = '';
  let newSiteUrl = '';
  let showAddForm = false;

  // Native Chrome Bookmarks
  let chromeBookmarks = [];
  let isExtensionMode = false;
  let bookmarkSearchQuery = '';
  let filteredBookmarks = [];

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

  // Load Chrome Bookmarks
  function loadChromeBookmarks() {
    if (typeof chrome !== 'undefined' && chrome.bookmarks) {
      isExtensionMode = true;
      // Fetch the full bookmarks tree
      chrome.bookmarks.getTree((treeNodes) => {
        // Flatten or pull Bookmarks Bar children
        const root = treeNodes[0];
        let items = [];
        
        function traverse(node) {
          if (node.url) {
            items.push({ name: node.title, url: node.url });
          }
          if (node.children) {
            node.children.forEach(traverse);
          }
        }
        
        if (root && root.children) {
          // Traverse through all main bookmark folders (Bookmarks Bar, Other, etc.)
          root.children.forEach(traverse);
        }
        chromeBookmarks = items;
        filterBookmarks();
      });
    } else {
      isExtensionMode = false;
      // Mock bookmarks for dev preview
      chromeBookmarks = [
        { name: 'Svelte Docs', url: 'https://svelte.dev' },
        { name: 'Vite Guide', url: 'https://vite.dev' },
        { name: 'Tailwind CSS Docs', url: 'https://tailwindcss.com' },
        { name: 'MDN Web Docs', url: 'https://developer.mozilla.org' },
        { name: 'Rust Documentation', url: 'https://doc.rust-lang.org' }
      ];
      filterBookmarks();
    }
  }

  function filterBookmarks() {
    if (!bookmarkSearchQuery.trim()) {
      filteredBookmarks = chromeBookmarks.slice(0, 15); // Show first 15 default
    } else {
      const q = bookmarkSearchQuery.toLowerCase();
      filteredBookmarks = chromeBookmarks.filter(b => 
        b.name.toLowerCase().includes(q) || b.url.toLowerCase().includes(q)
      ).slice(0, 15);
    }
  }

  $: {
    bookmarkSearchQuery;
    if (chromeBookmarks.length > 0) {
      filterBookmarks();
    }
  }

  onMount(() => {
    loadPinned();
    loadChromeBookmarks();
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
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-xl bg-cyan-500/10 flex items-center justify-center">
        <i class="fa fa-bookmark text-cyan-400 text-lg"></i>
      </div>
      <div>
        <h2 class="text-base font-semibold text-white">
          {#if isExtensionMode}Chrome Bookmarks{:else}Browser Bookmarks{/if}
        </h2>
        <p class="text-xs text-slate-500">Quick search active links</p>
      </div>
    </div>

    <!-- Search Input -->
    <div class="relative mb-3">
      <input type="text" bind:value={bookmarkSearchQuery} placeholder="Search bookmarks..." 
             class="w-full bg-white/5 border border-white/5 rounded-xl pl-9 pr-4 py-2.5 text-xs text-white placeholder-slate-500 focus:border-cyan-500/50 outline-none transition"
      />
      <i class="fa fa-search absolute left-3 top-3 text-slate-500 text-xs"></i>
    </div>

    <!-- Bookmarks List -->
    <div class="space-y-1.5 overflow-y-auto max-h-[160px] pr-1 scrollbar-thin">
      {#each filteredBookmarks as bookmark}
        <a href={bookmark.url} target="_blank" rel="noopener noreferrer"
           class="flex items-center justify-between p-2 hover:bg-slate-900/50 border border-transparent hover:border-white/5 rounded-xl transition text-xs group"
        >
          <span class="text-slate-300 group-hover:text-white truncate font-medium max-w-[180px]" title={bookmark.name}>
            {bookmark.name || 'Untitled'}
          </span>
          <span class="text-[10px] text-slate-500 truncate max-w-[150px] font-mono group-hover:text-cyan-400/80">
            {bookmark.url.replace(/^https?:\/\/(www\.)?/, '')}
          </span>
        </a>
      {:else}
        <p class="text-xs text-slate-500 text-center py-4">No matching bookmarks found.</p>
      {/each}
    </div>
  </div>
</div>
