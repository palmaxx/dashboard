<script>
  import { onMount } from 'svelte'

  export let wallpapers = []
  export let currentWallpaper = ''
  export let wallpaperFit = 'fill'
  export let wallpaperPosition = 'center'
  export let themes = []
  export let currentAccentThemeId = ''
  export let onSelect = () => {}
  export let onFitChange = () => {}
  export let onPositionChange = () => {}
  export let onThemeSelect = () => {}
  export let onClose = () => {}

  let dialogElement
  let customUrl = ''
  let fileError = ''

  $: currentPosition = wallpaperPosition === 'top'
    ? 'center top'
    : wallpaperPosition === 'bottom'
      ? 'center bottom'
      : 'center center'

  function applyCustom() {
    const url = customUrl.trim()
    if (!url) return
    fileError = ''
    onSelect(url)
    customUrl = ''
  }

  function handleUpload(event) {
    const file = event.target.files?.[0]
    if (!file) return
    if (file.size > 2 * 1024 * 1024) {
      fileError = 'That image is larger than 2 MB. Choose a smaller file.'
      event.target.value = ''
      return
    }

    fileError = ''
    const reader = new FileReader()
    reader.onload = result => {
      if (result.target?.result) onSelect(result.target.result)
    }
    reader.onerror = () => {
      fileError = 'The image could not be read. Try another file.'
    }
    reader.readAsDataURL(file)
  }

  function handleCancel(event) {
    event.preventDefault()
    onClose()
  }

  onMount(() => {
    if (dialogElement?.showModal) dialogElement.showModal()
    else dialogElement?.setAttribute('open', '')

    return () => {
      if (dialogElement?.open) dialogElement.close()
    }
  })
</script>

<dialog
  bind:this={dialogElement}
  class="backdrop"
  aria-labelledby="wallpaper-title"
  on:cancel={handleCancel}
  on:click|self={onClose}
>
  <section class="picker glass-strong">
    <header class="picker-header">
      <div>
        <h2 id="wallpaper-title">Personalize dashboard</h2>
        <p>Preview the full artwork, then tune how it fills your screen.</p>
      </div>
      <button class="close-button" type="button" on:click={onClose} aria-label="Close personalization">
        <svg viewBox="0 0 24 24" aria-hidden="true"><path d="m7 7 10 10M17 7 7 17"></path></svg>
      </button>
    </header>

    <div class="picker-body">
      <section class="artwork-area" aria-labelledby="artwork-title">
        <div class="section-heading">
          <div>
            <h3 id="artwork-title">Wallpaper</h3>
            <p>Full screen is recommended. Whole image preserves every edge.</p>
          </div>

          <div class="fit-switch" role="group" aria-label="Wallpaper sizing">
            <button
              type="button"
              class:active={wallpaperFit === 'fill'}
              aria-pressed={wallpaperFit === 'fill'}
              on:click={() => onFitChange('fill')}
            >Full screen</button>
            <button
              type="button"
              class:active={wallpaperFit === 'fit'}
              aria-pressed={wallpaperFit === 'fit'}
              on:click={() => onFitChange('fit')}
            >Whole image</button>
          </div>
        </div>

        <div class="current-preview" aria-label="Current wallpaper preview">
          <img class="preview-ambient" src={currentWallpaper} alt="" />
          <img
            class="preview-artwork"
            class:fill={wallpaperFit === 'fill'}
            src={currentWallpaper}
            alt=""
            style={`object-position: ${currentPosition}`}
          />
          <span>{wallpaperFit === 'fit' ? 'Whole image' : `Full screen · ${wallpaperPosition}`}</span>
        </div>

        {#if wallpaperFit === 'fill'}
          <div class="position-row">
            <span>Image focus</span>
            <div class="position-switch" role="group" aria-label="Wallpaper focal position">
              {#each ['top', 'center', 'bottom'] as position}
                <button
                  type="button"
                  class:active={wallpaperPosition === position}
                  aria-pressed={wallpaperPosition === position}
                  on:click={() => onPositionChange(position)}
                >{position}</button>
              {/each}
            </div>
          </div>
        {/if}

        <div class="wallpaper-grid">
          {#each wallpapers as wallpaper}
            <button
              class="wallpaper"
              class:active={currentWallpaper === wallpaper.url}
              type="button"
              on:click={() => onSelect(wallpaper.url)}
              aria-pressed={currentWallpaper === wallpaper.url}
            >
              <span class="wallpaper-preview">
                <img class="preview-ambient" src={wallpaper.url} alt="" loading="lazy" />
                <img class="preview-artwork" src={wallpaper.url} alt="" loading="lazy" />
              </span>
              <span class="wallpaper-name">{wallpaper.name}</span>
              {#if currentWallpaper === wallpaper.url}
                <svg class="selected-check" viewBox="0 0 24 24" aria-hidden="true">
                  <path d="m7 12 3 3 7-7"></path>
                </svg>
              {/if}
            </button>
          {/each}
        </div>
      </section>

      <section class="theme-area" aria-labelledby="accent-theme-title">
        <div class="section-heading">
          <div>
            <h3 id="accent-theme-title">Accent color</h3>
            <p>Used for actions, selected states, and live metrics.</p>
          </div>
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

      <section class="custom-area" aria-labelledby="custom-title">
        <div class="section-heading">
          <div>
            <h3 id="custom-title">Your image</h3>
            <p>Upload a local file or use a direct image URL.</p>
          </div>
        </div>

        <div class="custom-controls">
          <label class="upload">
            <svg viewBox="0 0 24 24" aria-hidden="true">
              <path d="M12 16V4m0 0L7 9m5-5 5 5"></path>
              <path d="M4 15v4a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-4"></path>
            </svg>
            <span>Upload image</span>
            <input class="sr-only" type="file" accept="image/*" on:change={handleUpload} />
          </label>

          <form class="url-row" on:submit|preventDefault={applyCustom}>
            <label class="sr-only" for="wallpaper-url">Image URL</label>
            <input id="wallpaper-url" class="input" type="url" inputmode="url" placeholder="Paste an image URL" bind:value={customUrl} />
            <button class="btn btn-accent" type="submit">Use URL</button>
          </form>
        </div>

        {#if fileError}
          <p class="file-error" role="alert">{fileError}</p>
        {:else}
          <p class="hint">Uploaded images stay in this browser. Maximum file size: 2 MB.</p>
        {/if}
      </section>
    </div>

    <footer class="picker-footer">
      <p>Changes are saved automatically on this device.</p>
      <button class="btn btn-accent" type="button" on:click={onClose}>Done</button>
    </footer>
  </section>
</dialog>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: var(--z-backdrop);
    width: 100%;
    max-width: none;
    height: 100%;
    max-height: none;
    padding: var(--sp-5);
    border: 0;
    background: rgb(2 5 12 / 0.74);
  }

  .backdrop[open] {
    display: grid;
    place-items: center;
  }

  .backdrop::backdrop {
    background: rgb(2 5 12 / 0.74);
  }

  .picker {
    z-index: var(--z-dialog);
    display: flex;
    width: min(54rem, 100%);
    max-height: calc(100dvh - 2.5rem);
    flex-direction: column;
    overflow: hidden;
    border-radius: var(--radius-lg);
  }

  .picker-header,
  .picker-footer {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-4);
    padding: var(--sp-4) var(--sp-5);
    border-bottom: 0.0625rem solid var(--glass-border);
    background: rgb(5 10 22 / 0.34);
  }

  .picker-header h2 {
    font-size: 1.25rem;
    font-weight: 700;
    line-height: 1.2;
  }

  .picker-header p,
  .picker-footer p {
    margin-top: var(--sp-1);
    color: var(--text-secondary);
    font-size: 0.8125rem;
  }

  .close-button {
    display: grid;
    width: 2.75rem;
    height: 2.75rem;
    flex: 0 0 auto;
    place-items: center;
    border: 0.0625rem solid var(--glass-border);
    border-radius: var(--radius-md);
    background: rgb(255 255 255 / 0.05);
    color: var(--text-secondary);
  }

  .close-button:hover {
    border-color: var(--line-strong);
    background: rgb(255 255 255 / 0.1);
    color: white;
  }

  .close-button svg,
  .upload svg {
    width: 1.125rem;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 2;
  }

  .picker-body {
    overflow-y: auto;
  }

  .artwork-area,
  .theme-area,
  .custom-area {
    padding: var(--sp-5);
  }

  .theme-area,
  .custom-area {
    border-top: 0.0625rem solid var(--glass-border);
  }

  .section-heading {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--sp-4);
    margin-bottom: var(--sp-4);
  }

  .section-heading h3 {
    color: var(--text-primary);
    font-size: 1rem;
    font-weight: 650;
  }

  .section-heading p {
    margin-top: var(--sp-1);
    color: var(--text-secondary);
    font-size: 0.8125rem;
  }

  .fit-switch,
  .position-switch {
    display: inline-flex;
    padding: 0.1875rem;
    border: 0.0625rem solid var(--glass-border);
    border-radius: var(--radius-md);
    background: rgb(3 7 15 / 0.46);
  }

  .fit-switch button,
  .position-switch button {
    min-height: 2.5rem;
    padding: var(--sp-1) var(--sp-3);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: 0.8125rem;
    font-weight: 650;
    text-transform: capitalize;
  }

  .fit-switch button:hover,
  .position-switch button:hover {
    color: white;
  }

  .fit-switch button.active,
  .position-switch button.active {
    background: var(--accent-strong);
    color: white;
  }

  .current-preview,
  .wallpaper-preview {
    position: relative;
    display: block;
    overflow: hidden;
    background: var(--bg);
  }

  .current-preview {
    width: 100%;
    aspect-ratio: 16 / 7;
    border: 0.0625rem solid var(--glass-border);
    border-radius: var(--radius-md);
  }

  .current-preview > img,
  .wallpaper-preview > img {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  .preview-ambient {
    transform: scale(1.12);
    filter: blur(1rem) saturate(1.15);
    object-fit: cover;
    opacity: 0.72;
  }

  .preview-artwork {
    object-fit: contain;
  }

  .preview-artwork.fill {
    object-fit: cover;
  }

  .current-preview > span {
    position: absolute;
    right: var(--sp-2);
    bottom: var(--sp-2);
    padding: var(--sp-1) var(--sp-2);
    border-radius: var(--radius-sm);
    background: rgb(3 7 15 / 0.72);
    color: white;
    font-size: 0.75rem;
    font-weight: 650;
    text-transform: capitalize;
  }

  .position-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--sp-4);
    margin-top: var(--sp-3);
  }

  .position-row > span {
    color: var(--text-secondary);
    font-size: 0.8125rem;
    font-weight: 600;
  }

  .wallpaper-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--sp-3);
    margin-top: var(--sp-4);
  }

  .wallpaper {
    position: relative;
    display: grid;
    min-width: 0;
    grid-template-columns: 5rem minmax(0, 1fr);
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-2);
    border: 0.0625rem solid transparent;
    border-radius: var(--radius-md);
    background: rgb(255 255 255 / 0.045);
    color: var(--text-secondary);
    text-align: left;
    transition: border-color 180ms ease, background-color 180ms ease, color 180ms ease;
  }

  .wallpaper:hover {
    border-color: var(--glass-border);
    background: rgb(255 255 255 / 0.08);
    color: white;
  }

  .wallpaper.active {
    border-color: var(--accent);
    background: var(--accent-soft);
    color: white;
  }

  .wallpaper-preview {
    width: 5rem;
    aspect-ratio: 16 / 10;
    border-radius: var(--radius-sm);
  }

  .wallpaper-name {
    overflow: hidden;
    padding-right: var(--sp-5);
    font-size: 0.8125rem;
    font-weight: 650;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .selected-check,
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
    stroke: white;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 2.5;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(7rem, 1fr));
    gap: var(--sp-2);
  }

  .theme-option {
    position: relative;
    display: flex;
    min-width: 0;
    min-height: 4.5rem;
    flex-direction: column;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--sp-2);
    padding: var(--sp-3);
    border: 0.0625rem solid transparent;
    border-radius: var(--radius-md);
    background: rgb(255 255 255 / 0.045);
    cursor: pointer;
    transition: border-color 180ms ease, background-color 180ms ease;
  }

  .theme-option:hover {
    border-color: var(--glass-border);
    background: rgb(255 255 255 / 0.08);
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
    height: 1.125rem;
    border: 0.0625rem solid rgb(255 255 255 / 0.26);
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
    opacity: 0;
    transition: opacity 180ms ease;
  }

  .theme-option.active .theme-check {
    opacity: 1;
  }

  .custom-controls {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: var(--sp-3);
  }

  .upload {
    display: inline-flex;
    min-height: 2.75rem;
    align-items: center;
    gap: var(--sp-2);
    padding: var(--sp-2) var(--sp-4);
    border: 0.0625rem solid var(--glass-border);
    border-radius: var(--radius-md);
    background: rgb(255 255 255 / 0.05);
    color: var(--text-primary);
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 650;
  }

  .upload:hover {
    border-color: var(--line-strong);
    background: rgb(255 255 255 / 0.09);
  }

  .upload:focus-within {
    outline: 0.1875rem solid var(--focus-ring);
    outline-offset: 0.1875rem;
  }

  .url-row {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: var(--sp-2);
  }

  .hint,
  .file-error {
    margin-top: var(--sp-3);
    color: var(--text-secondary);
    font-size: 0.75rem;
  }

  .file-error {
    color: var(--red);
  }

  .picker-footer {
    border-top: 0.0625rem solid var(--glass-border);
    border-bottom: 0;
  }

  @media (max-width: 42rem) {
    .backdrop {
      padding: var(--sp-2);
    }

    .picker {
      max-height: calc(100dvh - 1rem);
    }

    .section-heading,
    .position-row {
      align-items: flex-start;
      flex-direction: column;
    }

    .current-preview {
      aspect-ratio: 16 / 9;
    }

    .wallpaper-grid,
    .custom-controls {
      grid-template-columns: 1fr;
    }

    .theme-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .picker-footer p {
      display: none;
    }

    .picker-footer .btn {
      width: 100%;
    }
  }

  @media (max-width: 28rem) {
    .url-row {
      grid-template-columns: 1fr;
    }
  }
</style>
