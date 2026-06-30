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
      <p class="label">Projects</p>
      <h2>Tasks & Notes</h2>
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
  .active-badge {
    font-size: 12px;
    color: var(--text-secondary);
    padding: var(--sp-1) var(--sp-2);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
  }

  .project-list {
    display: flex;
    flex-direction: column;
  }

  .project {
    border-bottom: 1px solid var(--glass-border);
  }

  .project:last-child { border-bottom: none; }

  .project-header {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-3) var(--sp-5);
    background: var(--surface);
  }

  .color-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .project-header h3 {
    flex: 1;
    font-size: 14px;
    font-weight: 600;
    margin: 0;
  }

  .count {
    font-size: 11px;
    color: var(--text-tertiary);
  }

  .add-row {
    display: flex;
    gap: var(--sp-2);
    padding: var(--sp-2) var(--sp-5);
    border-bottom: 1px solid var(--glass-border);
  }

  .add-row .input { flex: 1; }

  .mode-toggle {
    width: 32px;
    height: 32px;
    display: grid;
    place-items: center;
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: 14px;
    background: var(--surface);
  }

  .entries {
    display: flex;
    flex-direction: column;
  }

  .empty-inline {
    font-size: 12px;
    color: var(--text-tertiary);
    padding: var(--sp-3) var(--sp-5);
  }
</style>
