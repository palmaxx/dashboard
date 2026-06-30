<script>
  export let activeView = 'hardware'
  export let onViewChange = () => {}
</script>

<nav class="sidebar" aria-label="Dashboard navigation">
  <button
    class:active={activeView === 'hardware'}
    on:click={() => onViewChange('hardware')}
    aria-pressed={activeView === 'hardware'}
  >
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    <span>Hardware</span>
  </button>
  <button
    class:active={activeView === 'bookmarks'}
    on:click={() => onViewChange('bookmarks')}
    aria-pressed={activeView === 'bookmarks'}
  >
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"/></svg>
    <span>Bookmarks</span>
  </button>
</nav>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
    padding: var(--sp-4) var(--sp-3);
    border-right: 1px solid var(--glass-border);
    background: rgba(10, 15, 20, 0.6);
  }

  button {
    display: flex;
    width: 100%;
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-3) var(--sp-3);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 14px;
    font-weight: 600;
    text-align: left;
    transition: all 180ms ease;
  }

  button:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  button.active {
    background: var(--accent-soft);
    border-color: rgba(94, 234, 212, 0.2);
    color: var(--text-primary);
    box-shadow: inset 3px 0 0 var(--accent);
  }

  button svg {
    flex-shrink: 0;
    opacity: 0.7;
  }

  button.active svg {
    opacity: 1;
    color: var(--accent);
  }
</style>
