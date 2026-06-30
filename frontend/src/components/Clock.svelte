<script>
  import { onMount } from 'svelte';

  export let connectionStatus = 'loading';

  let timeString = '';
  let dateString = '';
  let greeting = '';

  function updateClock() {
    const now = new Date();
    timeString = now.toLocaleTimeString([], {
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
    });
    dateString = now.toLocaleDateString('en-US', {
      weekday: 'long',
      month: 'long',
      day: 'numeric',
    });

    const hour = now.getHours();
    if (hour < 5) {
      greeting = 'Night';
    } else if (hour < 12) {
      greeting = 'Morning';
    } else if (hour < 17) {
      greeting = 'Afternoon';
    } else if (hour < 22) {
      greeting = 'Evening';
    } else {
      greeting = 'Night';
    }
  }

  onMount(() => {
    updateClock();
    const interval = setInterval(updateClock, 1000);
    return () => clearInterval(interval);
  });
</script>

<div class="clock-face">
  <div class="clock-kicker">
    <span
      class="clock-dot"
      class:online={connectionStatus === 'online'}
      class:offline={connectionStatus === 'offline'}
    ></span>
    <span>Good {greeting}</span>
  </div>
  <h1>{timeString}</h1>
  <div class="date-line">
    <span></span>
    <p>{dateString}</p>
    <span></span>
  </div>
</div>
