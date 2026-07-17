<script>
  import { addEntry, toggleEntry, updateEntry, deleteEntry } from '../lib/api.js'
  import EntryRow from './EntryRow.svelte'

  export let projects = []
  export let compact = false
  export let onRefresh = () => {}

  let newTexts = {}
  let newModes = {}

  function isTodo(pid) { return newModes[pid] !== false }
  function toggleMode(pid) { newModes = { ...newModes, [pid]: !isTodo(pid) } }

  async function handleAdd(pid) {
    const text = (newTexts[pid] || '').trim()
    if (!text) return
    await addEntry(pid, text, isTodo(pid))
    newTexts = { ...newTexts, [pid]: '' }
    onRefresh()
  }

  async function handleToggle(pid, line) {
    await toggleEntry(pid, line)
    onRefresh()
  }

  async function handleEdit(pid, line, oldText) {
    const text = prompt('Edit:', oldText)
    if (!text || text === oldText) return
    await updateEntry(pid, line, text)
    onRefresh()
  }

  async function handleDelete(pid, line) {
    if (!confirm('Delete this entry?')) return
    await deleteEntry(pid, line)
    onRefresh()
  }

  function activeCount(entries) {
    return entries.filter(e => e.kind === 'todo' && !e.done).length
  }

  $: totalActive = projects.reduce((sum, p) => sum + activeCount(p.entries || []), 0)
</script>

<section class="card">
  <div class="card-header">
    <div>
      <h2>Projects</h2>
      <p>Tasks and notes from repotasks</p>
    </div>
    <span class="active-badge">{totalActive} active</span>
  </div>

  {#if projects.length === 0}
    <div class="empty-state">
      <p>No projects found. Import via repotasks.</p>
    </div>
  {:else}
    <div class="project-list">
      {#each projects as pd}
        <article class="project">
          <header class="project-header">
            <span class="color-dot" style="background: {pd.project.color}"></span>
            <h3>{pd.project.name}</h3>
            <span class="count">{activeCount(pd.entries || [])} todos</span>
          </header>

          {#if !compact}
            <form class="add-row" on:submit|preventDefault={() => handleAdd(pd.project.id)}>
              <button type="button" class="mode-toggle" on:click={() => toggleMode(pd.project.id)} title={isTodo(pd.project.id) ? 'Todo' : 'Note'}>
                {isTodo(pd.project.id) ? '☑' : '□'}
              </button>
              <input
                class="input"
                type="text"
                placeholder="Add {isTodo(pd.project.id) ? 'todo' : 'note'}…"
                value={newTexts[pd.project.id] || ''}
                on:input={(e) => (newTexts = { ...newTexts, [pd.project.id]: e.target.value })}
              />
              <button class="btn btn-accent" type="submit">Add</button>
            </form>

            <div class="entries">
              {#each pd.entries || [] as entry}
                <EntryRow
                  {entry}
                  projectColor={pd.project.color}
                  onToggle={() => handleToggle(pd.project.id, entry.line)}
                  onEdit={() => handleEdit(pd.project.id, entry.line, entry.text)}
                  onDelete={() => handleDelete(pd.project.id, entry.line)}
                />
              {:else}
                <p class="empty-inline">No entries yet.</p>
              {/each}
            </div>
          {/if}
        </article>
      {/each}
    </div>
  {/if}
</section>

<style>
  .card-header {
    min-height: 4.75rem;
    border-bottom: 0.0625rem solid var(--glass-border);
    background: var(--glass-subtle);
    color: var(--text-primary);
  }

  .card-header p {
    margin-top: var(--sp-1);
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .active-badge {
    padding: var(--sp-1) var(--sp-2);
    border: 0.0625rem solid var(--glass-border);
    border-radius: var(--radius-sm);
    background: rgb(255 255 255 / 0.05);
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
  }

  .project-list {
    display: flex;
    flex-direction: column;
  }

  .project {
    border-bottom: 0.0625rem solid var(--line);
  }

  .project:last-child { border-bottom: none; }

  .project-header {
    display: flex;
    min-height: 3.5rem;
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-2) var(--sp-5);
    background: transparent;
  }

  .color-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .project-header h3 {
    flex: 1;
    overflow: hidden;
    font-size: 0.875rem;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .count {
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
  }

  .add-row {
    display: flex;
    gap: var(--sp-2);
    padding: var(--sp-2) var(--sp-5);
    border-bottom: 0.0625rem solid var(--line);
  }

  .add-row .input { flex: 1; }

  .mode-toggle {
    width: 2.75rem;
    height: 2.75rem;
    display: grid;
    place-items: center;
    border: 0.0625rem solid var(--line);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    background: var(--surface-raised);
    font-size: 0.875rem;
  }

  .entries {
    display: flex;
    flex-direction: column;
  }

  .empty-inline {
    padding: var(--sp-3) var(--sp-5);
    color: var(--text-secondary);
    font-size: 0.875rem;
  }
</style>
