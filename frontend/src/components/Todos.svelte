<script>
  import { onMount, onDestroy } from 'svelte';

  let projectsData = [];
  let loading = true;

  async function fetchProjects() {
    try {
      const res = await fetch('http://127.0.0.1:9999/api/projects');
      if (res.ok) {
        projectsData = await res.json();
      }
    } catch (err) {
      console.error("Failed to fetch projects", err);
    } finally {
      loading = false;
    }
  }

  let intervalId;
  onMount(() => {
    fetchProjects();
    intervalId = setInterval(fetchProjects, 5000);
  });
  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
  });

  async function toggleTodo(projectId, line) {
    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/toggle/${line}`, { method: 'POST' });
      fetchProjects();
    } catch (err) { console.error(err); }
  }

  async function deleteTodo(projectId, line) {
    if (!confirm('Delete this entry?')) return;
    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/delete/${line}`, { method: 'DELETE' });
      fetchProjects();
    } catch (err) { console.error(err); }
  }

  async function editTodo(projectId, line, oldText) {
    const text = prompt('Edit text:', oldText);
    if (!text || text === oldText) return;
    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/update/${line}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text })
      });
      fetchProjects();
    } catch (err) { console.error(err); }
  }

  let newTexts = {}; // map projectId -> new text string
  let newIsTodo = {}; // map projectId -> bool

  async function addEntry(projectId) {
    const text = (newTexts[projectId] || '').trim();
    if (!text) return;
    const isTodo = newIsTodo[projectId] !== false; // default true
    try {
      await fetch(`http://127.0.0.1:9999/api/projects/${projectId}/add`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text, is_todo: isTodo })
      });
      newTexts[projectId] = '';
      fetchProjects();
    } catch (err) { console.error(err); }
  }

  function toggleIsTodo(projectId) {
    newIsTodo[projectId] = !(newIsTodo[projectId] !== false);
  }

</script>

<div class="space-y-6 mt-6 select-none">
  {#if loading && projectsData.length === 0}
    <div class="glass rounded-2xl p-5 text-center text-slate-500">
      <i class="fa fa-spinner fa-spin text-2xl mb-2"></i>
      <p>Loading projects...</p>
    </div>
  {:else if projectsData.length === 0}
    <div class="glass rounded-2xl p-5 text-center text-slate-500">
      <i class="fa fa-clipboard text-3xl mb-2 block opacity-30"></i>
      <p>No projects found. Import one in repotasks first.</p>
    </div>
  {:else}
    {#each projectsData as pd}
      <div class="glass rounded-2xl p-5 hover:border-slate-700/50 transition-all duration-300">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-4 h-4 rounded-full shadow-sm" style="background-color: {pd.project.color}; shadow-color: {pd.project.color}"></div>
          <h2 class="text-base font-semibold text-white">{pd.project.name}</h2>
          <span class="text-xs text-slate-500 ml-auto font-medium">
            {pd.entries.filter(e => e.kind === 'todo' && !e.done).length} active todos
          </span>
        </div>

        <!-- Add Entry Form -->
        <div class="flex gap-2 mb-4">
          <button on:click={() => toggleIsTodo(pd.project.id)}
                  class="bg-slate-900 border border-white/5 rounded-xl px-3 py-2 text-xs text-slate-300 hover:text-white hover:bg-white/5 transition flex items-center justify-center w-10"
                  title="Toggle Todo/Note"
          >
            {#if newIsTodo[pd.project.id] !== false}
              <i class="fa fa-check-square-o text-teal-400"></i>
            {:else}
              <i class="fa fa-sticky-note-o text-amber-400"></i>
            {/if}
          </button>
          
          <input type="text" bind:value={newTexts[pd.project.id]} on:keydown={(e) => e.key === 'Enter' && addEntry(pd.project.id)}
                 placeholder="Add to {pd.project.name}..." 
                 class="flex-1 bg-white/5 border border-white/5 rounded-xl px-4 py-2 text-xs text-white placeholder-slate-500 outline-none transition"
          />
          <button on:click={() => addEntry(pd.project.id)} 
                  class="bg-white/10 hover:bg-white/20 text-white font-bold text-xs rounded-xl px-4 py-2 transition"
          >
            Add
          </button>
        </div>

        <!-- Entries List -->
        <div class="space-y-1.5 max-h-64 overflow-y-auto pr-1">
          {#each pd.entries as entry}
            <div class="flex items-start gap-3 px-3 py-2.5 bg-slate-950/20 hover:bg-slate-900/50 border border-white/5 hover:border-slate-800 rounded-xl transition group">
              
              {#if entry.kind === 'todo'}
                <button on:click={() => toggleTodo(pd.project.id, entry.line)} 
                        class="w-5 h-5 mt-0.5 rounded-full border-2 flex shrink-0 items-center justify-center transition"
                        style="{entry.done ? `background-color: ${pd.project.color}; border-color: ${pd.project.color};` : 'border-color: #64748b;'} "
                >
                  {#if entry.done}
                    <i class="fa fa-check text-[10px] text-slate-950"></i>
                  {/if}
                </button>
              {:else}
                <div class="w-5 h-5 mt-0.5 flex shrink-0 items-center justify-center opacity-50">
                  <i class="fa fa-minus text-[10px] text-slate-400"></i>
                </div>
              {/if}
              
              <div class="flex-1 min-w-0 flex flex-col pt-0.5">
                <span class="text-xs break-words whitespace-pre-wrap transition-all duration-300 {entry.done ? 'line-through text-slate-500' : 'text-slate-200'}">
                  {entry.text}
                </span>
                {#if entry.timestamp}
                  <span class="text-[9px] text-slate-500 mt-0.5">{entry.timestamp}</span>
                {/if}
              </div>
              
              <!-- Actions -->
              <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition duration-300 shrink-0">
                <button on:click={() => editTodo(pd.project.id, entry.line, entry.text)} 
                        class="w-6 h-6 rounded-lg hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition"
                >
                  <i class="fa fa-pencil text-[10px]"></i>
                </button>
                <button on:click={() => deleteTodo(pd.project.id, entry.line)} 
                        class="w-6 h-6 rounded-lg hover:bg-rose-500/10 flex items-center justify-center text-slate-400 hover:text-rose-400 transition"
                >
                  <i class="fa fa-trash text-[10px]"></i>
                </button>
              </div>
            </div>
          {:else}
             <div class="text-center py-4 text-slate-500 text-xs">
               <i class="fa fa-check-circle-o text-2xl mb-1 block opacity-30"></i>
               No tasks here!
             </div>
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>
