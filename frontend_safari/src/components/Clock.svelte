<script>
  import { onMount } from 'svelte';

  let timeString = '';
  let dateString = '';
  let greeting = '';

  export let connectionStatus = 'loading';

  function update() {
    const now = new Date();
    const hours = String(now.getHours()).padStart(2, '0');
    const minutes = String(now.getMinutes()).padStart(2, '0');
    timeString = `${hours}:${minutes}`;

    const options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };
    dateString = now.toLocaleDateString('en-US', options);

    const h = now.getHours();
    if (h < 5) {
      greeting = 'Night';
    } else if (h < 12) {
      greeting = 'Morning';
    } else if (h < 17) {
      greeting = 'Afternoon';
    } else if (h < 22) {
      greeting = 'Evening';
    } else {
      greeting = 'Night';
    }
  }

  onMount(() => {
    update();
    const interval = setInterval(update, 1000);
    return () => clearInterval(interval);
  });
</script>

<div class="text-center select-none py-8">
  <div class="flex items-center justify-center gap-2 mb-3">
    <span class="w-2 h-2 rounded-full animate-pulse transition-all duration-500
      {connectionStatus === 'online' ? 'bg-emerald-400 shadow-[0_0_10px_rgba(52,211,153,0.8)]' : 
       connectionStatus === 'offline' ? 'bg-amber-400 shadow-[0_0_10px_rgba(251,191,36,0.8)]' : 
       'bg-slate-500 shadow-[0_0_6px_rgba(100,116,139,0.5)]'}"
    ></span>
    <span class="text-xs text-slate-400 font-semibold tracking-widest uppercase">
      Good {greeting}
    </span>
  </div>
  <h1 class="text-7xl md:text-8xl font-bold font-mono tracking-tight text-white time-glow">
    {timeString}
  </h1>
  <p class="text-xl md:text-2xl text-slate-300 font-light mt-3">
    {dateString}
  </p>
</div>
