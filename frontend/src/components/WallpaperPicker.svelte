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

  function handleUpload(event) {
    const file = event.target.files?.[0]
    if (!file) return
    if (file.size > 2 * 1024 * 1024) {
      alert('Image too large. Choose an image under 2 MB.')
      return
    }
    const reader = new FileReader()
    reader.onload = result => {
      if (result.target?.result) onSelect(result.target.result)
    }
    reader.readAsDataURL(file)
  }

  function handleKeydown(event) {
    if (event.key === 'Escape') onClose()
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<dialog open class="backdrop" aria-labelledby="wallpaper-title" on:click|self={onClose}>
  <section class="picker">
    <header>
      <div>
        <h2 id="wallpaper-title">Choose wallpaper</h2>
        <p>Set the image behind your clock</p>
      </div>
      <button class="close-button" type="button" on:click={onClose} aria-label="Close wallpaper picker">
        <svg viewBox="0 0 24 24" aria-hidden="true"><path d="m7 7 10 10M17 7 7 17"></path></svg>
      </button>
    </header>

    <div class="wallpaper-grid">
      {#each wallpapers as wallpaper}
        <button
          class="wallpaper"
          class:active={currentWallpaper === wallpaper.url}
          type="button"
          on:click={() => onSelect(wallpaper.url)}
          aria-pressed={currentWallpaper === wallpaper.url}
        >
          <img src={wallpaper.url} alt="" loading="lazy" />
          <span>{wallpaper.name}</span>
        </button>
      {/each}
    </div>

    <div class="custom-area">
      <label class="upload">
        <span>Upload an image</span>
        <input type="file" accept="image/*" on:change={handleUpload} />
      </label>

      <form class="url-row" on:submit|preventDefault={applyCustom}>
        <label class="sr-only" for="wallpaper-url">Image URL</label>
        <input id="wallpaper-url" class="input" type="url" inputmode="url" placeholder="Paste an image URL" bind:value={customUrl} />
        <button class="btn btn-accent" type="submit">Apply</button>
      </form>
      <p class="hint">Uploaded images stay in this browser. Maximum file size: 2 MB.</p>
    </div>
  </section>
</dialog>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: var(--z-backdrop);
    display: grid;
    width: 100%;
    max-width: none;
    height: 100%;
    max-height: none;
    place-items: center;
    padding: var(--sp-5);
    border: 0;
    background: rgb(3 7 15 / 0.8);
  }

  .picker {
    z-index: var(--z-dialog);
    width: min(40rem, 100%);
    max-height: calc(100vh - 2.5rem);
    overflow-y: auto;
    border: 0.0625rem solid var(--line-strong);
    border-radius: var(--radius-lg);
    background: var(--surface);
  }

  header {
    display: flex;
    min-height: 4.75rem;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-4);
    padding: var(--sp-4) var(--sp-5);
    background: var(--accent-strong);
    color: white;
  }

  header h2 {
    font-size: 1.25rem;
    font-weight: 700;
  }

  header p {
    margin-top: var(--sp-1);
    color: rgb(255 255 255 / 0.82);
    font-size: 0.875rem;
  }

  .close-button {
    display: grid;
    width: 2.75rem;
    height: 2.75rem;
    place-items: center;
    border: 0.0625rem solid rgb(255 255 255 / 0.4);
    border-radius: var(--radius-md);
    background: rgb(3 7 15 / 0.2);
    color: white;
  }

  .close-button:hover {
    background: rgb(3 7 15 / 0.4);
  }

  .close-button svg {
    width: 1.125rem;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-width: 2;
  }

  .wallpaper-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--sp-3);
    padding: var(--sp-5);
  }

  .wallpaper {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: var(--sp-2);
    padding: var(--sp-2);
    border: 0.125rem solid transparent;
    border-radius: var(--radius-md);
    background: var(--surface-raised);
    transition: border-color 180ms ease, background-color 180ms ease;
  }

  .wallpaper:hover {
    border-color: var(--line-strong);
    background: var(--surface-hover);
  }

  .wallpaper.active {
    border-color: var(--accent);
  }

  .wallpaper img {
    width: 100%;
    aspect-ratio: 16 / 9;
    border-radius: var(--radius-sm);
    object-fit: cover;
  }

  .wallpaper span {
    overflow: hidden;
    font-size: 0.875rem;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .custom-area {
    display: flex;
    flex-direction: column;
    gap: var(--sp-3);
    padding: var(--sp-5);
    border-top: 0.0625rem solid var(--line);
  }

  .upload {
    display: inline-flex;
    min-height: 2.75rem;
    align-items: center;
    align-self: flex-start;
    padding: var(--sp-2) var(--sp-4);
    border: 0.0625rem solid var(--line);
    border-radius: var(--radius-md);
    background: var(--surface-raised);
    color: var(--text-primary);
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 650;
  }

  .upload:hover {
    border-color: var(--line-strong);
    background: var(--surface-hover);
  }

  .upload input {
    display: none;
  }

  .url-row {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: var(--sp-2);
  }

  .hint {
    color: var(--text-secondary);
    font-size: 0.75rem;
  }

  @media (max-width: 36rem) {
    .wallpaper-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .url-row {
      grid-template-columns: 1fr;
    }
  }
</style>
