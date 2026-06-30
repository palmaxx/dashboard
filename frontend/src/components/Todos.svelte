<script>
  import { onMount, onDestroy } from 'svelte';

  export let variant = 'compact';

  let projectsData = [];
  let loading = true;
  let errorMessage = '';
  let intervalId;
  let newTexts = {};
  let newIsTodo = {};

  async function fetchProjects() {
    try {
      const response = await fetch('http://127.0.0.1:9999/api/projects', { cache: 'no-store' });
      if (!response.ok) {
        throw new Error(`Projects API returned ${response.status}`);
      }
      projectsData = await response.json();
      errorMessage = '';
    } catch (error) {
      errorMessage = 'Projects are unavailable until the local daemon is running.';
    } finally {
      loading = false;
    }
  }

  function activeCount(entries) {
    return entries.filter((entry) => entry.kind === 'todo' && !entry.done).length;
  }

  function isTodo(projectId) {
    return newIsTodo[projectId] !== false;
  }

  function setEntryText(projectId, value) {
    newTexts = { ...newTexts, [projectId]: value };
  }

  function toggleIsTodo(projectId) {
    newIsTodo = { ...newIsTodo, [projectId]: !isTodo(projectId) };
  }

  async function addEntry(projectId) {
    const text = (newTexts[projectId] || '').trim();
    if (!text) return;

    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/add`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text, is_todo: isTodo(projectId) }),
      });
      newTexts = { ...newTexts, [projectId]: '' };
      fetchProjects();
    } catch (error) {
      errorMessage = 'Could not add entry. Check the local daemon.';
    }
  }

  async function toggleTodo(projectId, line) {
    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/toggle/${line}`, {
        method: 'POST',
      });
      fetchProjects();
    } catch (error) {
      errorMessage = 'Could not update todo state.';
    }
  }

  async function deleteTodo(projectId, line) {
    if (!confirm('Delete entry?')) return;

    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/delete/${line}`, {
        method: 'DELETE',
      });
      fetchProjects();
    } catch (error) {
      errorMessage = 'Could not delete entry.';
    }
  }

  async function editTodo(projectId, line, oldText) {
    const text = prompt('Edit text:', oldText);
    if (!text || text === oldText) return;

    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/update/${line}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text }),
      });
      fetchProjects();
    } catch (error) {
      errorMessage = 'Could not edit entry.';
    }
  }

  $: totalActive = projectsData.reduce((sum, projectData) => sum + activeCount(projectData.entries || []), 0);

  onMount(() => {
    fetchProjects();
    intervalId = setInterval(fetchProjects, 5000);
  });

  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
  });
</script>

<section class="projects-deck" class:full={variant === 'full'}>
  <div class="module-header">
    <div>
      <p>Today / Projects</p>
      <h2>Projects</h2>
    </div>
    <span class="module-count">{totalActive} active</span>
  </div>

  {#if loading && projectsData.length === 0}
    <div class="empty-state">
      <i class="fa fa-circle-o-notch fa-spin" aria-hidden="true"></i>
      <p>Loading projects...</p>
    </div>
  {:else if errorMessage}
    <div class="empty-state">
      <i class="fa fa-plug" aria-hidden="true"></i>
      <p>{errorMessage}</p>
    </div>
  {:else if projectsData.length === 0}
    <div class="empty-state">
      <i class="fa fa-check-square-o" aria-hidden="true"></i>
      <p>No projects found. Import one in repotasks first.</p>
    </div>
  {:else}
    <div class="project-list">
      {#each projectsData as projectData}
        <article class="project-panel">
          <header>
            <span class="project-color" style={`background-color: ${projectData.project.color}`}></span>
            <div>
              <h3>{projectData.project.name}</h3>
              <p>{activeCount(projectData.entries || [])} active todos</p>
            </div>
          </header>

          <form class="entry-form" on:submit|preventDefault={() => addEntry(projectData.project.id)}>
            <button
              type="button"
              class:note={!isTodo(projectData.project.id)}
              title={isTodo(projectData.project.id) ? 'Adding todo' : 'Adding note'}
              on:click={() => toggleIsTodo(projectData.project.id)}
            >
              <i class="fa {isTodo(projectData.project.id) ? 'fa-check-square-o' : 'fa-sticky-note-o'}" aria-hidden="true"></i>
            </button>
            <input
              type="text"
              value={newTexts[projectData.project.id] || ''}
              aria-label={`Add entry to ${projectData.project.name}`}
              on:input={(event) => setEntryText(projectData.project.id, event.target.value)}
            />
            <button type="submit">Add entry</button>
          </form>

          <div class="entry-list">
            {#each projectData.entries as entry}
              <div class="entry-row" class:done={entry.done}>
                {#if entry.kind === 'todo'}
                  <button
                    type="button"
                    class="check-control"
                    class:checked={entry.done}
                    title={entry.done ? 'Mark as incomplete' : 'Mark as complete'}
                    style={entry.done ? `--project-color: ${projectData.project.color}` : ''}
                    on:click={() => toggleTodo(projectData.project.id, entry.line)}
                  >
                    {#if entry.done}
                      <i class="fa fa-check" aria-hidden="true"></i>
                    {/if}
                  </button>
                {:else}
                  <span class="note-marker">
                    <i class="fa fa-minus" aria-hidden="true"></i>
                  </span>
                {/if}

                <div class="entry-copy">
                  <p>{entry.text}</p>
                  {#if entry.timestamp}
                    <small>{entry.timestamp}</small>
                  {/if}
                </div>

                <div class="entry-actions">
                  <button type="button" title="Edit" aria-label="Edit entry" on:click={() => editTodo(projectData.project.id, entry.line, entry.text)}>
                    <i class="fa fa-pencil" aria-hidden="true"></i>
                  </button>
                  <button type="button" title="Delete" aria-label="Delete entry" on:click={() => deleteTodo(projectData.project.id, entry.line)}>
                    <i class="fa fa-trash" aria-hidden="true"></i>
                  </button>
                </div>
              </div>
            {:else}
              <p class="empty-inline">No entries here yet.</p>
            {/each}
          </div>
        </article>
      {/each}
    </div>
  {/if}
</section>
