<script>
  import { onMount } from 'svelte';

  export let variant = 'compact';

  const generatedIcons = ['fa-globe', 'fa-link', 'fa-external-link', 'fa-laptop'];
  const generatedColors = ['#66d9ef', '#8be28b', '#ffcc66', '#b08cff', '#ff7a90'];

  let pinnedSites = [];
  let chromeBookmarks = [];
  let filteredBookmarks = [];
  let isExtensionMode = false;
  let bookmarkSearchQuery = '';
  let showAddForm = false;
  let newSiteName = '';
  let newSiteUrl = '';

  function hostname(url) {
    try {
      return new URL(url).hostname.replace(/^www\./, '');
    } catch {
      return url.replace(/^https?:\/\/(www\.)?/, '');
    }
  }

  function normalizeSite(site, index) {
    if (!site?.url) return null;
    const name = site.name?.trim() || hostname(site.url);

    return {
      id: site.id || `${name}-${index}`,
      name,
      url: site.url,
      icon: site.icon || generatedIcons[index % generatedIcons.length],
      color: site.color || generatedColors[index % generatedColors.length],
    };
  }

  function loadPinned() {
    try {
      const stored = localStorage.getItem('dashPinnedSites');
      const savedSites = stored ? JSON.parse(stored) : [];
      
      // Auto-cleanup legacy dummy pins if they were cached
      if (savedSites.length === 4 && savedSites.every(s => ['google', 'youtube', 'github', 'reddit'].includes(s.id))) {
        localStorage.removeItem('dashPinnedSites');
        pinnedSites = [];
      } else {
        pinnedSites = Array.isArray(savedSites) ? savedSites.map(normalizeSite).filter(Boolean) : [];
      }
    } catch (error) {
      pinnedSites = [];
    }
    savePinned();
  }

  function savePinned() {
    try {
      localStorage.setItem('dashPinnedSites', JSON.stringify(pinnedSites));
    } catch (error) {
      console.warn('Unable to save pinned shortcuts', error);
    }
  }

  function addPinned() {
    const name = newSiteName.trim();
    let url = newSiteUrl.trim();
    if (!name || !url) return;

    if (!/^https?:\/\//i.test(url)) {
      url = `https://${url}`;
    }

    const nextIndex = pinnedSites.length;
    pinnedSites = [
      ...pinnedSites,
      {
        id: `${Date.now()}-${name}`,
        name,
        url,
        icon: generatedIcons[nextIndex % generatedIcons.length],
        color: generatedColors[nextIndex % generatedColors.length],
      },
    ];
    savePinned();
    newSiteName = '';
    newSiteUrl = '';
    showAddForm = false;
  }

  function removePinned(siteId) {
    pinnedSites = pinnedSites.filter((site) => site.id !== siteId);
    savePinned();
  }

  function loadChromeBookmarks() {
    if (typeof chrome !== 'undefined' && chrome.bookmarks) {
      isExtensionMode = true;
      chrome.bookmarks.getTree((treeNodes) => {
        const items = [];

        function traverse(node, folder = 'Bookmarks') {
          const nextFolder = node.title || folder;
          if (node.url) {
            items.push({ name: node.title || 'Untitled', url: node.url, folder });
          }
          if (node.children) {
            node.children.forEach((child) => traverse(child, nextFolder));
          }
        }

        treeNodes.forEach((node) => traverse(node));
        chromeBookmarks = items;
      });
    } else {
      isExtensionMode = false;
      chromeBookmarks = [];
    }
  }

  function filterBookmarks(bookmarks, query) {
    const limit = variant === 'full' ? 14 : 4;
    const trimmed = query.trim().toLowerCase();
    if (!trimmed) return bookmarks.slice(0, limit);

    return bookmarks
      .filter((bookmark) => {
        const name = bookmark.name?.toLowerCase() || '';
        const url = bookmark.url?.toLowerCase() || '';
        const folder = bookmark.folder?.toLowerCase() || '';
        return name.includes(trimmed) || url.includes(trimmed) || folder.includes(trimmed);
      })
      .slice(0, limit);
  }

  $: filteredBookmarks = filterBookmarks(chromeBookmarks, bookmarkSearchQuery);

  onMount(() => {
    loadPinned();
    loadChromeBookmarks();
  });
</script>

<section class="bookmarks-deck" class:full={variant === 'full'}>
  <div class="module-header">
    <div>
      <p>{isExtensionMode ? 'Chrome Bookmarks' : 'Extension API Unavailable'}</p>
      <h2>Bookmarks</h2>
    </div>
    <button type="button" class="icon-action" aria-label="Add pinned shortcut" title="Add pinned shortcut" on:click={() => (showAddForm = !showAddForm)}>
      <i class="fa fa-plus" aria-hidden="true"></i>
    </button>
  </div>

  <div class="shortcut-strip">
    <div class="strip-title">
      <i class="fa fa-thumb-tack" aria-hidden="true"></i>
      <span>Pinned Shortcuts</span>
    </div>

    {#if showAddForm}
      <form class="pin-form" on:submit|preventDefault={addPinned}>
        <input type="text" bind:value={newSiteName} aria-label="Pinned shortcut name" />
        <input type="text" bind:value={newSiteUrl} aria-label="Pinned shortcut URL" />
        <button type="submit">Add</button>
      </form>
    {/if}

    {#if pinnedSites.length}
      <div class="pin-grid">
        {#each pinnedSites as site}
          <a href={site.url} target="_blank" rel="noopener noreferrer" class="pin-tile" style={`--site-color: ${site.color}`}>
            <button
              type="button"
              aria-label={`Remove ${site.name}`}
              title={`Remove ${site.name}`}
              on:click|preventDefault|stopPropagation={() => removePinned(site.id)}
            >
              <i class="fa fa-times" aria-hidden="true"></i>
            </button>
            <span>
              <i class="fa {site.icon}" aria-hidden="true"></i>
            </span>
            <strong>{site.name}</strong>
          </a>
        {/each}
      </div>
    {:else}
      <p class="pin-empty">No pinned shortcuts yet.</p>
    {/if}
  </div>

  <div class="bookmark-search-panel">
    <div class="search-row">
      <i class="fa fa-search" aria-hidden="true"></i>
      <input type="text" bind:value={bookmarkSearchQuery} aria-label="Search bookmarks" />
      <span>{chromeBookmarks.length}</span>
    </div>

    <div class="bookmark-list">
      {#each filteredBookmarks as bookmark}
        <a href={bookmark.url} target="_blank" rel="noopener noreferrer" class="bookmark-row">
          <span class="favicon-dot">{bookmark.name?.slice(0, 1) || '?'}</span>
          <strong title={bookmark.name}>{bookmark.name || 'Untitled'}</strong>
          <small title={bookmark.url}>{hostname(bookmark.url)}</small>
          <em>{bookmark.folder || 'Bookmarks'}</em>
          <i class="fa fa-star-o" aria-hidden="true"></i>
        </a>
      {:else}
        <p class="empty-inline">
          {isExtensionMode ? 'No matching bookmarks found.' : 'Chrome bookmarks are available when the dashboard is installed as an extension.'}
        </p>
      {/each}
    </div>
  </div>
</section>
