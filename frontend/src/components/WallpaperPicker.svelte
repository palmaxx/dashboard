<script>
  export let wallpapers = []
  export let currentWallpaper = ''
  export let themes = []
  export let currentAccentThemeId = ''
  export let onSelect = () => {}
  export let onThemeSelect = () => {}
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
        <h2 id="wallpaper-title">Personalize dashboard</h2>
        <p>Choose a wallpaper and accent color</p>
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

    <section class="theme-area" aria-labelledby="accent-theme-title">
      <div class="theme-heading">
        <h3 id="accent-theme-title">Accent color</h3>
        <p>Changes buttons, active states, and highlights.</p>
      </div>

      <div class="theme-grid" role="radiogroup" aria-labelledby="accent-theme-title">
        {#each themes as theme}
          <label
            class="theme-option"
            class:active={currentAccentThemeId === theme.id}
            style={`--swatch: ${theme.accent}; --swatch-soft: ${theme.soft}`}
          >
            <input
              class="sr-only"
              type="radio"
              name="accent-theme"
              value={theme.id}
              checked={currentAccentThemeId === theme.id}
              on:change={() => onThemeSelect(theme.id)}
            />
            <span class="theme-swatch" aria-hidden="true"></span>
            <span class="theme-name">{theme.name}</span>
            <svg class="theme-check" viewBox="0 0 24 24" aria-hidden="true">
              <path d="m7 12 3 3 7-7"></path>
            </svg>
          </label>
        {/each}
      </div>
    </section>

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

  .theme-area {
    padding: var(--sp-5);
    border-top: 0.0625rem solid var(--line);
  }

  .theme-heading {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--sp-4);
    margin-bottom: var(--sp-3);
  }

  .theme-heading h3 {
    color: var(--text-primary);
    font-size: 1rem;
    font-weight: 650;
  }

  .theme-heading p {
    color: var(--text-secondary);
    font-size: 0.75rem;
    text-align: right;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(6.25rem, 1fr));
    gap: var(--sp-2);
  }

  .theme-option {
    position: relative;
    display: flex;
    min-width: 0;
    min-height: 5rem;
    flex-direction: column;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--sp-2);
    padding: var(--sp-3);
    border: 0.125rem solid transparent;
    border-radius: var(--radius-md);
    background: var(--surface-raised);
    cursor: pointer;
    transition: border-color 180ms ease, background-color 180ms ease;
  }

  .theme-option:hover {
    border-color: var(--line-strong);
    background: var(--surface-hover);
  }

  .theme-option:focus-within {
    outline: 0.1875rem solid var(--focus-ring);
    outline-offset: 0.125rem;
  }

  .theme-option.active {
    border-color: var(--accent);
    background: var(--accent-soft);
  }

  .theme-swatch {
    width: 2rem;
    height: 1.25rem;
    border: 0.0625rem solid rgb(255 255 255 / 0.28);
    border-radius: var(--radius-sm);
    background: var(--swatch);
  }

  .theme-name {
    max-width: 100%;
    overflow: hidden;
    color: var(--text-primary);
    font-size: 0.75rem;
    font-weight: 650;
    line-height: 1.2;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .theme-check {
    position: absolute;
    top: var(--sp-2);
    right: var(--sp-2);
    width: 1.25rem;
    height: 1.25rem;
    padding: 0.125rem;
    border-radius: 50%;
    background: var(--accent-strong);
    fill: none;
    opacity: 0;
    stroke: white;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 2.5;
    transition: opacity 180ms ease;
  }

  .theme-option.active .theme-check {
    opacity: 1;
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

    .theme-heading {
      align-items: flex-start;
      flex-direction: column;
      gap: var(--sp-1);
    }

    .theme-heading p {
      text-align: left;
    }

    .url-row {
      grid-template-columns: 1fr;
    }
  }
</style>
