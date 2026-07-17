<script>
  import { onDestroy } from 'svelte'

  export let url = ''
  export let fitMode = 'fill'
  export let focalPosition = 'center'

  let activeUrl = url
  let outgoingUrl = ''
  let transitionTimer

  $: if (url && url !== activeUrl) {
    outgoingUrl = activeUrl
    activeUrl = url
    clearTimeout(transitionTimer)
    transitionTimer = setTimeout(() => {
      outgoingUrl = ''
    }, 220)
  }

  $: objectPosition = focalPosition === 'top'
    ? 'center top'
    : focalPosition === 'bottom'
      ? 'center bottom'
      : 'center center'

  onDestroy(() => clearTimeout(transitionTimer))
</script>

<div class="wallpaper-stage" aria-hidden="true">
  {#if outgoingUrl}
    <div class="wallpaper-frame outgoing">
      <img class="ambient-layer" src={outgoingUrl} alt="" />
      <img
        class="artwork-layer"
        class:fill={fitMode === 'fill'}
        src={outgoingUrl}
        alt=""
        style={`object-position: ${objectPosition}`}
      />
    </div>
  {/if}

  {#key activeUrl}
    <div class="wallpaper-frame incoming">
      <img class="ambient-layer" src={activeUrl} alt="" />
      <img
        class="artwork-layer"
        class:fill={fitMode === 'fill'}
        src={activeUrl}
        alt=""
        style={`object-position: ${objectPosition}`}
      />
    </div>
  {/key}
  <div class="wallpaper-scrim"></div>
</div>

<style>
  .wallpaper-stage {
    position: fixed;
    inset: 0;
    z-index: 0;
    min-height: 100dvh;
    overflow: hidden;
    background: var(--bg);
    pointer-events: none;
  }

  .wallpaper-frame,
  .wallpaper-scrim {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  .wallpaper-frame {
    z-index: 0;
  }

  .wallpaper-frame.incoming {
    z-index: 2;
    animation: wallpaper-in 220ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .wallpaper-frame.outgoing {
    z-index: 1;
    animation: wallpaper-out 220ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .ambient-layer,
  .artwork-layer {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  .ambient-layer {
    transform: scale(1.1);
    filter: blur(2rem) saturate(1.18);
    object-fit: cover;
    opacity: 0.76;
  }

  .artwork-layer {
    padding: clamp(0rem, 1.5vw, 1rem);
    object-fit: contain;
  }

  .artwork-layer.fill {
    padding: 0;
    object-fit: cover;
  }

  .wallpaper-scrim {
    z-index: 3;
    background:
      linear-gradient(180deg, rgb(2 5 12 / 0.28) 0%, rgb(2 5 12 / 0.12) 34%, rgb(2 5 12 / 0.64) 100%),
      linear-gradient(90deg, rgb(2 5 12 / 0.38) 0%, transparent 58%);
  }

  @keyframes wallpaper-in {
    from {
      opacity: 0;
    }
  }

  @keyframes wallpaper-out {
    to {
      opacity: 0;
    }
  }

  @media (max-width: 40rem) {
    .ambient-layer {
      filter: blur(1.5rem) saturate(1.12);
    }

    .artwork-layer {
      padding: 0;
    }

    .wallpaper-scrim {
      background:
        linear-gradient(180deg, rgb(2 5 12 / 0.24) 0%, rgb(2 5 12 / 0.24) 32%, rgb(2 5 12 / 0.74) 100%),
        linear-gradient(90deg, rgb(2 5 12 / 0.34) 0%, transparent 100%);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .wallpaper-frame.incoming,
    .wallpaper-frame.outgoing {
      animation: none;
    }
  }
</style>
