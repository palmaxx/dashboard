<script>
  export let wallpapers = []
  export let currentWallpaper = ''
  export let onSelect = () => {}
  export let onClose = () => {}

  let customUrl = ''

  function applyCustom() {
    const url = customUrl.trim()
    if (!url) return
    onSelect(url)
    customUrl = ''
  }

  function handleUpload(e) {
    const file = e.target.files?.[0]
    if (!file) return
    if (file.size > 2 * 1024 * 1024) {
      alert('Image too large. Please choose under 2MB.')
      return
    }
    const reader = new FileReader()
    reader.onload = (ev) => { if (ev.target?.result) onSelect(ev.target.result) }
    reader.readAsDataURL(file)
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="backdrop" on:click|self={onClose}>
  <section class="picker card">
    <div class="card-header">
      <div>
        <p class="label">Visual System</p>
        <h2>Choose Wallpaper</h2>
      </div>
      <button class="btn-icon" on:click={onClose} aria-label="Close">✕</button>
    </div>

    <div class="wp-grid">
      {#each wallpapers as wp}
        <button class="wp-thumb" class:active={currentWallpaper === wp.url} on:click={() => onSelect(wp.url)}>
          <img src={wp.url} alt={wp.name} loading="lazy" />
          <span>{wp.name}</span>
        </button>
      {/each}
    </div>

    <div class="custom-area">
      <label class="upload-label">
        <span>Upload Image</span>
        <input type="file" accept="image/*" on:change={handleUpload} />
      </label>
      <div class="url-row">
        <input class="input" type="text" placeholder="Paste image URL…" bind:value={customUrl} on:keydown={(e) => e.key === 'Enter' && applyCustom()} />
        <button class="btn btn-accent" on:click={applyCustom}>Apply</button>
      </div>
      <p class="hint">Uploads stored in browser. Keep files under 2MB.</p>
    </div>
  </section>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: grid;
    place-items: center;
    background: rgba(0,0,0,0.6);
    backdrop-filter: blur(4px);
  }

  .picker {
    width: min(560px, calc(100vw - 40px));
    max-height: calc(100vh - 80px);
    overflow-y: auto;
  }

  .wp-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--sp-3);
    padding: var(--sp-4);
  }

  .wp-thumb {
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
    padding: var(--sp-2);
    border: 2px solid transparent;
    border-radius: var(--radius-md);
    background: var(--surface);
    cursor: pointer;
    transition: border-color 200ms ease;
  }

  .wp-thumb.active {
    border-color: var(--accent);
  }

  .wp-thumb:hover {
    border-color: rgba(255,255,255,0.2);
  }

  .wp-thumb img {
    width: 100%;
    aspect-ratio: 16/9;
    object-fit: cover;
    border-radius: var(--radius-sm);
  }

  .wp-thumb span {
    font-size: 12px;
    color: var(--text-secondary);
    text-align: center;
  }

  .custom-area {
    padding: var(--sp-4);
    border-top: 1px solid var(--glass-border);
    display: flex;
    flex-direction: column;
    gap: var(--sp-3);
  }

  .upload-label {
    display: inline-flex;
    align-items: center;
    gap: var(--sp-2);
    font-size: 13px;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .upload-label input { display: none; }

  .upload-label span {
    padding: var(--sp-2) var(--sp-3);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    transition: background 200ms;
  }

  .upload-label:hover span {
    background: var(--surface-hover);
  }

  .url-row {
    display: flex;
    gap: var(--sp-2);
  }

  .hint {
    font-size: 11px;
    color: var(--text-tertiary);
    margin: 0;
  }
</style>
