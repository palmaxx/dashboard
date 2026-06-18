<script>
  import { onMount } from 'svelte';

  let todos = [];
  let newTodoText = '';
  let newTodoPriority = 'low';
  let filter = 'all';

  function loadTodos() {
    try {
      const stored = localStorage.getItem('dashTodos');
      if (stored) {
        todos = JSON.parse(stored);
      }
    } catch (e) {
      todos = [];
    }
  }

  function saveTodos() {
    localStorage.setItem('dashTodos', JSON.stringify(todos));
  }

  function addTodo() {
    const text = newTodoText.trim();
    if (!text) return;
    todos = [
      {
        id: Date.now(),
        text,
        done: false,
        priority: newTodoPriority,
        created: new Date().toISOString()
      },
      ...todos
    ];
    saveTodos();
    newTodoText = '';
  }

  function toggleTodo(id) {
    todos = todos.map(t => t.id === id ? { ...t, done: !t.done } : t);
    saveTodos();
  }

  function deleteTodo(id) {
    todos = todos.filter(t => t.id !== id);
    saveTodos();
  }

  function editTodo(id) {
    const todo = todos.find(t => t.id === id);
    if (!todo) return;
    const newText = prompt('Edit task:', todo.text);
    if (newText !== null && newText.trim()) {
      todos = todos.map(t => t.id === id ? { ...t, text: newText.trim() } : t);
      saveTodos();
    }
  }

  function clearCompleted() {
    todos = todos.filter(t => !t.done);
    saveTodos();
  }

  $: remainingCount = todos.filter(t => !t.done).length;

  $: filteredTodos = (() => {
    if (filter === 'active') return todos.filter(t => !t.done);
    if (filter === 'completed') return todos.filter(t => t.done);
    return todos;
  })();

  onMount(() => {
    loadTodos();
  });
</script>

<div class="glass rounded-2xl p-5 mt-6 text-left select-none hover:border-slate-700/50 transition-all duration-300">
  <div class="flex items-center justify-between mb-4">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-xl bg-teal-500/10 flex items-center justify-center">
        <i class="fa fa-check-square-o text-teal-400 text-lg"></i>
      </div>
      <div>
        <h2 class="text-base font-semibold text-white">Focus Checklist</h2>
        <p class="text-xs text-slate-500">{remainingCount} task{remainingCount === 1 ? '' : 's'} remaining</p>
      </div>
    </div>
    
    <div class="flex items-center gap-3">
      <select bind:value={filter} 
              class="bg-slate-900 border border-white/5 rounded-lg px-2.5 py-1.5 text-xs text-slate-300 focus:border-teal-500/50 outline-none cursor-pointer"
      >
        <option value="all">All</option>
        <option value="active">Active</option>
        <option value="completed">Completed</option>
      </select>
      {#if todos.some(t => t.done)}
        <button on:click={clearCompleted} 
                class="text-xs text-slate-500 hover:text-rose-400 transition font-semibold"
        >
          Clear Completed
        </button>
      {/if}
    </div>
  </div>
  
  <!-- Add Task Form -->
  <div class="flex gap-3 mb-4">
    <input type="text" bind:value={newTodoText} on:keydown={(e) => e.key === 'Enter' && addTodo()}
           placeholder="Add a new task..." 
           class="flex-1 bg-white/5 border border-white/5 rounded-xl px-4 py-2.5 text-xs text-white placeholder-slate-500 focus:border-teal-500/50 outline-none transition"
    />
    <select bind:value={newTodoPriority} 
            class="bg-slate-900 border border-white/5 rounded-xl px-3 py-2.5 text-xs text-slate-300 focus:border-teal-500/50 outline-none cursor-pointer"
    >
      <option value="low">Low</option>
      <option value="medium">Medium</option>
      <option value="high">High</option>
    </select>
    <button on:click={addTodo} 
            class="bg-teal-500 hover:bg-teal-600 text-slate-950 font-bold text-xs rounded-xl px-5 py-2.5 transition flex items-center justify-center"
    >
      <i class="fa fa-plus text-sm"></i>
    </button>
  </div>
  
  <!-- Todos List -->
  <div class="space-y-1.5 max-h-64 overflow-y-auto pr-1">
    {#each filteredTodos as todo}
      <div class="flex items-center gap-3 px-4 py-3 bg-slate-950/20 hover:bg-slate-900/50 border border-white/5 hover:border-slate-800 rounded-xl transition group">
        
        <!-- Done Checkbox -->
        <button on:click={() => toggleTodo(todo.id)} 
                class="w-5 h-5 rounded-full border-2 flex items-center justify-center transition
                       {todo.done ? 'bg-teal-500 border-teal-500 text-slate-950' : 'border-slate-500 hover:border-teal-400 text-transparent'}"
        >
          <i class="fa fa-check text-[10px]"></i>
        </button>
        
        <!-- Text -->
        <span class="text-xs flex-1 truncate transition-all duration-300
                     {todo.done ? 'line-through text-slate-500' : 'text-slate-200'}"
        >
          {todo.text}
        </span>
        
        <!-- Priority Tag -->
        <span class="text-[9px] px-2 py-0.5 rounded-full font-bold uppercase border
                     {todo.priority === 'high' ? 'bg-rose-500/10 text-rose-400 border-rose-500/20' : 
                      todo.priority === 'medium' ? 'bg-amber-500/10 text-amber-400 border-amber-500/20' : 
                      'bg-emerald-500/10 text-emerald-400 border-emerald-500/20'}"
        >
          {todo.priority}
        </span>
        
        <!-- Actions (Edit/Delete) -->
        <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition duration-300">
          <button on:click={() => editTodo(todo.id)} 
                  class="w-6 h-6 rounded-lg hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition"
          >
            <i class="fa fa-pencil text-[10px]"></i>
          </button>
          <button on:click={() => deleteTodo(todo.id)} 
                  class="w-6 h-6 rounded-lg hover:bg-rose-500/10 flex items-center justify-center text-slate-400 hover:text-rose-400 transition"
          >
            <i class="fa fa-trash text-[10px]"></i>
          </button>
        </div>
      </div>
    {:else}
      <div class="text-center py-6 text-slate-500 text-xs">
        <i class="fa fa-clipboard text-3xl mb-2 block opacity-30"></i>
        <span>No focus tasks found. Add one above!</span>
      </div>
    {/each}
  </div>
</div>
