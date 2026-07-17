<script>
  export let entry = {}
  export let projectColor = 'var(--accent)'
  export let onToggle = () => {}
  export let onEdit = () => {}
  export let onDelete = () => {}
</script>

<div class="entry" class:done={entry.done}>
  {#if entry.kind === 'todo'}
    <button
      class="checkbox"
      class:checked={entry.done}
      style={entry.done ? `--check-color: ${projectColor}` : ''}
      on:click={onToggle}
      title={entry.done ? 'Mark incomplete' : 'Mark complete'}
    >
      {#if entry.done}✓{/if}
    </button>
  {:else}
    <span class="note-marker">—</span>
  {/if}

  <div class="entry-content">
    <p>{entry.text}</p>
    {#if entry.timestamp}
      <small>{entry.timestamp}</small>
    {/if}
  </div>

  <div class="entry-actions">
    <button on:click={onEdit} title="Edit" aria-label="Edit">✎</button>
    <button on:click={onDelete} title="Delete" aria-label="Delete">⌫</button>
  </div>
</div>

<style>
  .entry {
    display: flex;
    align-items: flex-start;
    gap: var(--sp-3);
    padding: var(--sp-2) var(--sp-5);
    border-bottom: 1px solid var(--glass-border);
    transition: background 150ms;
  }

  .entry:hover { background: var(--surface); }
  .entry:last-child { border-bottom: none; }

  .entry.done { opacity: 0.5; }

  .checkbox {
    width: 20px;
    height: 20px;
    display: grid;
    place-items: center;
    border: 1.5px solid var(--glass-border);
    border-radius: 4px;
    background: transparent;
    color: transparent;
    font-size: 11px;
    flex-shrink: 0;
    margin-top: 2px;
    transition: all 200ms ease;
  }

  .checkbox.checked {
    background: var(--check-color, var(--accent));
    border-color: var(--check-color, var(--accent));
    color: #fff;
  }

  .note-marker {
    width: 20px;
    text-align: center;
    color: var(--text-tertiary);
    flex-shrink: 0;
    margin-top: 2px;
  }

  .entry-content {
    flex: 1;
    min-width: 0;
  }

  .entry-content p {
    font-size: 13px;
    color: var(--text-primary);
    margin: 0;
    word-break: break-word;
  }

  .entry-content small {
    font-size: 11px;
    color: var(--text-tertiary);
    margin-top: 2px;
    display: block;
  }

  .entry-actions {
    display: flex;
    gap: var(--sp-1);
    opacity: 0;
    transition: opacity 150ms;
  }

  .entry:hover .entry-actions,
  .entry:focus-within .entry-actions { opacity: 1; }

  .entry-actions button {
    width: 24px;
    height: 24px;
    display: grid;
    place-items: center;
    border-radius: 4px;
    color: var(--text-tertiary);
    font-size: 12px;
    transition: all 150ms;
  }

  .entry-actions button:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }
</style>
