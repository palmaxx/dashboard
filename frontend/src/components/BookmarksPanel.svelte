<script>
  import { onMount } from 'svelte'

  export let compact = false

  const COLORS = ['#5eead4', '#4ade80', '#fbbf24', '#a78bfa', '#f87171']
  const ICONS = ['◎', '⌁', '↗', '▣']

  let pinned = []
  let chromeBookmarks = []
  let filtered = []
  let isExtension = false
  let query = ''
  let showAddForm = false
  let newName = ''
  let newUrl = ''

  function hostname(url) {
    try { return new URL(url).hostname.replace(/^www\./, '') }
    catch { return url }
  }

  function loadPinned() {
    try {
      const raw = localStorage.getItem('dashPins')
      pinned = raw ? JSON.parse(raw) : []
    } catch { pinned = [] }
  }

  function savePinned() {
    try { localStorage.setItem('dashPins', JSON.stringify(pinned)) }
    catch { /* ignore */ }
  }

  function addPin() {
    const name = newName.trim()
    let url = newUrl.trim()
    if (!name || !url) return
    if (!/^https?:\/\//i.test(url)) url = `https://${url}`
    const i = pinned.length
    pinned = [...pinned, { id: `${Date.now()}`, name, url, icon: ICONS[i % ICONS.length], color: COLORS[i % COLORS.length] }]
    savePinned()
    newName = ''
    newUrl = ''
    showAddForm = false
  }

  function removePin(id) {
    pinned = pinned.filter(p => p.id !== id)
    savePinned()
  }

  function loadChromeBookmarks() {
    if (typeof chrome !== 'undefined' && chrome.bookmarks) {
      isExtension = true
      chrome.bookmarks.getTree(tree => {
        const items = []
        function walk(node, folder = 'Bookmarks') {
          const f = node.title || folder
          if (node.url) items.push({ name: node.title || 'Untitled', url: node.url, folder })
          if (node.children) node.children.forEach(c => walk(c, f))
        }
        tree.forEach(n => walk(n))
        chromeBookmarks = items
      })
    }
  }

  $: {
    const limit = compact ? 6 : 16
    const q = query.trim().toLowerCase()
    filtered = q
      ? chromeBookmarks.filter(b => (b.name||'').toLowerCase().includes(q) || (b.url||'').toLowerCase().includes(q)).slice(0, limit)
      : chromeBookmarks.slice(0, limit)
  }

  onMount(() => {
    loadPinned()
    loadChromeBookmarks()
  })
</script>

<section class="card">
  <div class="card-header">
    <div>
      <p class="label">{isExtension ? 'Chrome Bookmarks' : 'Bookmarks'}</p>
      <h2>Bookmarks</h2>
    </div>
    <button class="btn-icon" on:click={() => (showAddForm = !showAddForm)} aria-label="Add shortcut" title="Add shortcut">+</button>
  </div>

  <div class="shortcuts">
    <div class="section-label">
      <span>⌖</span> Pinned Shortcuts
    </div>

    {#if showAddForm}
      <form class="add-form" on:submit|preventDefault={addPin}>
        <input class="input" type="text" placeholder="Name" bind:value={newName} />
        <input class="input" type="text" placeholder="URL" bind:value={newUrl} />
        <button class="btn btn-accent" type="submit">Add</button>
      </form>
    {/if}

    {#if pinned.length}
      <div class="pin-grid" class:compact>
        {#each pinned as pin}
          <a href={pin.url} target="_blank" rel="noopener noreferrer" class="pin" style="--pin-color: {pin.color}">
            <button class="pin-remove" on:click|preventDefault|stopPropagation={() => removePin(pin.id)} aria-label="Remove {pin.name}">✕</button>
            <span class="pin-icon">{pin.icon}</span>
            <strong>{pin.name}</strong>
          </a>
        {/each}
      </div>
    {:else}
      <p class="empty-hint">No pinned shortcuts. Click + to add one.</p>
    {/if}
  </div>

  <div class="bm-search">
    <div class="search-bar">
      <span class="search-icon">⌕</span>
      <input class="input" type="text" placeholder="Search bookmarks…" bind:value={query} />
      <span class="bm-count">{chromeBookmarks.length}</span>
    </div>

    <div class="bm-list">
      {#each filtered as bm}
        <a href={bm.url} target="_blank" rel="noopener noreferrer" class="bm-row">
          <span class="bm-initial">{(bm.name || '?')[0]}</span>
          <strong title={bm.name}>{bm.name || 'Untitled'}</strong>
          <small title={bm.url}>{hostname(bm.url)}</small>
          <em>{bm.folder || 'Bookmarks'}</em>
        </a>
      {:else}
        <p class="empty-hint">
          {isExtension ? 'No matching bookmarks.' : 'Chrome bookmarks available when installed as extension.'}
        </p>
      {/each}
    </div>
  </div>
</section>

<style>
  .shortcuts {
    padding: var(--sp-3) var(--sp-4);
    border-bottom: 1px solid var(--glass-border);
  }

  .section-label {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    font-size: 12px;
    font-weight: 700;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    margin-bottom: var(--sp-3);
  }

  .add-form {
    display: flex;
    gap: var(--sp-2);
    margin-bottom: var(--sp-3);
  }

  .add-form .input { flex: 1; }

  .pin-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--sp-2);
  }

  .pin-grid.compact {
    grid-template-columns: repeat(3, 1fr);
  }

  .pin {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--sp-1);
    padding: var(--sp-3) var(--sp-2);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-md);
    background: var(--surface);
    transition: all 200ms ease;
  }

  .pin:hover {
    border-color: color-mix(in srgb, var(--pin-color) 50%, transparent);
    background: var(--surface-hover);
    transform: translateY(-2px);
  }

  .pin-icon {
    display: grid;
    width: 30px;
    height: 30px;
    place-items: center;
    border-radius: var(--radius-sm);
    background: color-mix(in srgb, var(--pin-color) 20%, transparent);
    color: var(--pin-color);
    font-size: 16px;
  }

  .pin strong {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
    text-align: center;
  }

  .pin-remove {
    position: absolute;
    top: 4px;
    right: 4px;
    width: 18px;
    height: 18px;
    display: grid;
    place-items: center;
    border-radius: 4px;
    background: rgba(248, 113, 113, 0.15);
    border: 1px solid rgba(248, 113, 113, 0.2);
    color: var(--red);
    font-size: 9px;
    opacity: 0;
    transition: opacity 150ms;
  }

  .pin:hover .pin-remove { opacity: 1; }

  .empty-hint {
    font-size: 12px;
    color: var(--text-tertiary);
    padding: var(--sp-3) 0;
    text-align: center;
  }

  .bm-search {
    display: flex;
    flex-direction: column;
  }

  .search-bar {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    padding: var(--sp-3) var(--sp-4);
    border-bottom: 1px solid var(--glass-border);
  }

  .search-icon {
    color: var(--text-tertiary);
    font-size: 16px;
  }

  .search-bar .input {
    flex: 1;
    border: none;
    background: transparent;
    padding: 0;
    min-height: auto;
  }

  .search-bar .input:focus { border-color: transparent; }

  .bm-count {
    font-size: 11px;
    color: var(--text-tertiary);
    padding: 2px 6px;
    border: 1px solid var(--glass-border);
    border-radius: 4px;
  }

  .bm-list {
    max-height: 320px;
    overflow-y: auto;
  }

  .bm-row {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-2) var(--sp-4);
    border-bottom: 1px solid var(--glass-border);
    transition: background 150ms;
  }

  .bm-row:hover { background: var(--surface-hover); }
  .bm-row:last-child { border-bottom: none; }

  .bm-initial {
    width: 24px;
    height: 24px;
    display: grid;
    place-items: center;
    border-radius: 6px;
    background: var(--surface);
    font-size: 11px;
    font-weight: 700;
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .bm-row strong {
    flex: 1;
    font-size: 13px;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bm-row small {
    font-size: 11px;
    color: var(--text-tertiary);
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bm-row em {
    font-size: 10px;
    color: var(--text-tertiary);
    font-style: normal;
    max-width: 80px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
