<script>
  import { onMount } from 'svelte';

  let timeString = '';
  let dateString = '';
  let greeting = '';

  function update() {
    const now = new Date();
    const hours = String(now.getHours()).padStart(2, '0');
    const minutes = String(now.getMinutes()).padStart(2, '0');
    timeString = `${hours}:${minutes}`;

    const options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };
    dateString = now.toLocaleDateString('en-US', options);

    const h = now.getHours();
    if (h < 12) {
      greeting = 'Morning';
    } else if (h < 18) {
      greeting = 'Afternoon';
    } else {
      greeting = 'Evening';
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
    <span class="w-2 h-2 bg-emerald-400 rounded-full animate-pulse"></span>
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
