<script>
  import { onMount } from 'svelte'

  const COLORS = ['#3ddcff', '#56e39f', '#ffb84d', '#7d8cff', '#ff6b72']

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
    } catch {
      pinned = []
    }
  }

  function savePinned() {
    try { localStorage.setItem('dashPins', JSON.stringify(pinned)) }
    catch {}
  }

  function addPin() {
    const name = newName.trim()
    let url = newUrl.trim()
    if (!name || !url) return
    if (!/^https?:\/\//i.test(url)) url = 'https://' + url
    const index = pinned.length
    pinned = [...pinned, { id: String(Date.now()), name, url, color: COLORS[index % COLORS.length] }]
    savePinned()
    newName = ''
    newUrl = ''
    showAddForm = false
  }

  function removePin(id) {
    pinned = pinned.filter(pin => pin.id !== id)
    savePinned()
  }

  function loadChromeBookmarks() {
    if (typeof chrome !== 'undefined' && chrome.bookmarks) {
      isExtension = true
      chrome.bookmarks.getTree(tree => {
        const items = []
        function walk(node, folder = 'Bookmarks') {
          const nextFolder = node.title || folder
          if (node.url) items.push({ name: node.title || 'Untitled', url: node.url, folder })
          if (node.children) node.children.forEach(child => walk(child, nextFolder))
        }
        tree.forEach(node => walk(node))
        chromeBookmarks = items
      })
    }
  }

  $: {
    const normalized = query.trim().toLowerCase()
    const matches = normalized
      ? chromeBookmarks.filter(bookmark =>
          (bookmark.name || '').toLowerCase().includes(normalized) ||
          (bookmark.url || '').toLowerCase().includes(normalized)
        )
      : chromeBookmarks
    filtered = matches.slice(0, normalized ? 16 : 12)
  }

  onMount(() => {
    loadPinned()
    loadChromeBookmarks()
  })
</script>

<section class="card bookmarks-panel">
  <header class="panel-header">
    <div>
      <h2>Bookmarks</h2>
      <p>{isExtension ? chromeBookmarks.length + ' Chrome bookmarks' : 'Pinned shortcuts remain available'}</p>
    </div>
    <button
      class="add-button"
      type="button"
      on:click={() => (showAddForm = !showAddForm)}
      aria-expanded={showAddForm}
      aria-controls="add-shortcut-form"
    >
      <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 5v14M5 12h14"></path></svg>
      <span>Add</span>
    </button>
  </header>

  <div class="shortcuts">
    <div class="subhead">
      <h3>Pinned shortcuts</h3>
      <span>{pinned.length}</span>
    </div>

    {#if showAddForm}
      <form id="add-shortcut-form" class="add-form" on:submit|preventDefault={addPin}>
        <label>
          <span>Name</span>
          <input class="input" type="text" autocomplete="off" bind:value={newName} />
        </label>
        <label>
          <span>URL</span>
          <input class="input" type="url" inputmode="url" autocomplete="url" bind:value={newUrl} />
        </label>
        <button class="btn btn-accent" type="submit">Save shortcut</button>
      </form>
    {/if}

    {#if pinned.length}
      <div class="pin-grid">
        {#each pinned as pin}
          <div class="pin-wrap" style="--pin-color: {pin.color || COLORS[0]}">
            <a href={pin.url} title={pin.name} class="pin">
              <span class="pin-initial">{(pin.name || '?')[0].toUpperCase()}</span>
              <strong>{pin.name}</strong>
            </a>
            <button class="pin-remove" type="button" on:click={() => removePin(pin.id)} aria-label="Remove {pin.name}">
              <svg viewBox="0 0 24 24" aria-hidden="true"><path d="m7 7 10 10M17 7 7 17"></path></svg>
            </button>
          </div>
        {/each}
      </div>
    {:else}
      <button class="empty-action" type="button" on:click={() => (showAddForm = true)}>Add your first shortcut</button>
    {/if}
  </div>

  <div class="bookmark-search">
    <label class="search">
      <span class="sr-only">Search bookmarks</span>
      <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="11" cy="11" r="7"></circle><path d="m20 20-4-4"></path></svg>
      <input type="search" placeholder="Search bookmarks" bind:value={query} />
      <span class="count">{filtered.length}</span>
    </label>

    <div class="bookmark-list">
      {#each filtered as bookmark}
        <a href={bookmark.url} class="bookmark-row">
          <span class="bookmark-initial">{(bookmark.name || '?')[0].toUpperCase()}</span>
          <span class="bookmark-copy">
            <strong title={bookmark.name}>{bookmark.name || 'Untitled'}</strong>
            <small title={bookmark.url}>{hostname(bookmark.url)}</small>
          </span>
          <span class="folder">{bookmark.folder || 'Bookmarks'}</span>
        </a>
      {:else}
        <p class="bookmark-empty">
          {isExtension ? 'No bookmarks match this search.' : 'Chrome bookmarks appear in the installed extension.'}
        </p>
      {/each}
    </div>
  </div>
</section>

<style>
  .bookmarks-panel {
    background: transparent;
  }

  .panel-header {
    display: flex;
    min-height: 4.75rem;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-4);
    padding: var(--sp-4) var(--sp-5);
    border-bottom: 0.0625rem solid var(--glass-border);
    background: var(--glass-subtle);
    color: var(--text-primary);
  }

  .panel-header h2 {
    font-size: 1.25rem;
    font-weight: 700;
    line-height: 1.2;
  }

  .panel-header p {
    margin-top: var(--sp-1);
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .add-button {
    display: inline-flex;
    min-height: 2.75rem;
    align-items: center;
    gap: var(--sp-2);
    padding: var(--sp-2) var(--sp-3);
    border: 0.0625rem solid var(--glass-border);
    border-radius: var(--radius-md);
    background: rgb(255 255 255 / 0.05);
    color: var(--text-primary);
    font-size: 0.875rem;
    font-weight: 650;
  }

  .add-button:hover {
    border-color: var(--line-strong);
    background: rgb(255 255 255 / 0.1);
  }

  .add-button svg,
  .pin-remove svg,
  .search svg {
    width: 1rem;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 2;
  }

  .shortcuts {
    padding: var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
  }

  .subhead {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-3);
    margin-bottom: var(--sp-3);
  }

  .subhead h3 {
    font-size: 1rem;
    font-weight: 650;
  }

  .subhead span,
  .count {
    min-width: 1.75rem;
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
    text-align: right;
  }

  .add-form {
    display: grid;
    gap: var(--sp-3);
    margin-bottom: var(--sp-4);
  }

  .add-form label {
    display: grid;
    gap: var(--sp-1);
  }

  .add-form label > span {
    color: var(--text-secondary);
    font-size: 0.875rem;
    font-weight: 600;
  }

  .pin-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--sp-2);
  }

  .pin-wrap {
    position: relative;
    min-width: 0;
  }

  .pin {
    display: flex;
    min-height: 4.25rem;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--sp-1);
    padding: var(--sp-2);
    border: 0.0625rem solid var(--glass-border);
    border-radius: var(--radius-md);
    background: rgb(255 255 255 / 0.045);
    transition: border-color 180ms ease, background-color 180ms ease;
  }

  .pin:hover {
    border-color: var(--pin-color);
    background: var(--surface-hover);
  }

  .pin-initial {
    display: grid;
    width: 1.75rem;
    height: 1.75rem;
    place-items: center;
    border-radius: var(--radius-sm);
    background: var(--pin-color);
    color: var(--bg);
    font-size: 0.875rem;
    font-weight: 750;
  }

  .pin strong {
    max-width: 100%;
    overflow: hidden;
    font-size: 0.75rem;
    font-weight: 650;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .pin-remove {
    position: absolute;
    top: var(--sp-1);
    right: var(--sp-1);
    display: grid;
    width: 1.75rem;
    height: 1.75rem;
    place-items: center;
    border-radius: var(--radius-sm);
    background: rgb(3 7 15 / 0.86);
    color: var(--red);
    opacity: 0;
    transition: opacity 150ms ease;
  }

  .pin-wrap:hover .pin-remove,
  .pin-wrap:focus-within .pin-remove {
    opacity: 1;
  }

  .empty-action {
    width: 100%;
    min-height: 3.5rem;
    border: 0.0625rem dashed var(--line-strong);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 0.875rem;
    font-weight: 600;
  }

  .empty-action:hover {
    border-color: var(--accent);
    color: var(--text-primary);
  }

  .bookmark-search {
    display: flex;
    flex-direction: column;
  }

  .search {
    display: grid;
    min-height: 3.75rem;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: var(--sp-3);
    padding: 0 var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
    color: var(--text-secondary);
  }

  .search input {
    min-width: 0;
    border: 0;
    outline: 0;
    background: transparent;
    color: var(--text-primary);
    font-size: 1rem;
  }

  .search input::placeholder {
    color: var(--text-tertiary);
    opacity: 1;
  }

  .bookmark-list {
    display: flex;
    flex-direction: column;
  }

  .bookmark-row {
    display: grid;
    min-height: 3.5rem;
    grid-template-columns: auto minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-2) var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
    transition: background-color 150ms ease;
  }

  .bookmark-row:hover {
    background: rgb(255 255 255 / 0.065);
  }

  .bookmark-row:last-child {
    border-bottom: 0;
  }

  .bookmark-initial {
    display: grid;
    width: 2rem;
    height: 2rem;
    place-items: center;
    border-radius: var(--radius-sm);
    background: rgb(255 255 255 / 0.08);
    color: var(--text-primary);
    font-size: 0.75rem;
    font-weight: 700;
  }

  .bookmark-copy {
    display: flex;
    min-width: 0;
    flex-direction: column;
  }

  .bookmark-copy strong,
  .bookmark-copy small,
  .folder {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bookmark-copy strong {
    font-size: 0.875rem;
    font-weight: 600;
  }

  .bookmark-copy small,
  .folder {
    color: var(--text-secondary);
    font-size: 0.75rem;
  }

  .folder {
    max-width: 6rem;
  }

  .bookmark-empty {
    padding: var(--sp-6) var(--sp-5);
    color: var(--text-secondary);
    font-size: 0.875rem;
    text-align: center;
    text-wrap: pretty;
  }

  @media (hover: none) {
    .pin-remove {
      opacity: 1;
    }
  }

  @media (max-width: 28rem) {
    .pin-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .folder {
      display: none;
    }
  }
</style>
