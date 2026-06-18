<script>
  import { onMount } from 'svelte';

  let hardware = null;
  let status = 'loading'; // 'loading' | 'online' | 'offline'
  let lastUpdated = null;

  async function fetchStats() {
    try {
      const res = await fetch('http://127.0.0.1:9999/api/sysinfo', { cache: 'no-store' });
      if (res.ok) {
        hardware = await res.json();
        status = 'online';
        lastUpdated = new Date(hardware.timestamp);
      } else {
        throw new Error('Not OK');
      }
    } catch (e) {
      status = 'offline';
    }
  }

  onMount(() => {
    fetchStats();
    const interval = setInterval(fetchStats, 3000);
    return () => clearInterval(interval);
  });
</script>

<!-- Hardware Daemon Connection Status -->
<div class="flex items-center justify-between px-4 py-2 rounded-xl mb-4 text-xs font-semibold select-none
  {status === 'online' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' : 
   status === 'offline' ? 'bg-amber-500/10 text-amber-400 border border-amber-500/20' : 
   'bg-slate-800 text-slate-400 border border-slate-700'}"
>
  <div class="flex items-center gap-2">
    <span class="w-2 h-2 rounded-full {status === 'online' ? 'bg-emerald-400 animate-pulse' : status === 'offline' ? 'bg-amber-400' : 'bg-slate-400'}"></span>
    <span>
      {#if status === 'online'}
        Local Daemon: Connected
      {:else if status === 'offline'}
        Local Daemon: Offline (Run Rust daemon or tray app for real system stats)
      {:else}
        Connecting to local daemon...
      {/if}
    </span>
  </div>
  {#if status === 'online' && lastUpdated}
    <span class="text-slate-400 font-normal">
      Last update: {lastUpdated.toLocaleTimeString()}
    </span>
  {/if}
</div>

<!-- Main Stats Grid -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
  
  <!-- CPU CARD -->
  <div class="glass rounded-2xl p-5 hover:-translate-y-1 hover:shadow-2xl hover:border-violet-500/30 transition-all duration-300">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-violet-500/10 flex items-center justify-center">
          <i class="fa fa-microchip text-violet-400 text-lg"></i>
        </div>
        <div>
          <div class="text-xs text-slate-400 uppercase tracking-wider font-semibold">CPU</div>
          <div class="text-lg font-bold text-white">
            {status === 'online' && hardware ? Math.round(hardware.cpu.usagePercent) : 0}%
          </div>
        </div>
      </div>
      {#if status === 'online' && hardware && hardware.cpu.temperatureC}
        <span class="text-xs text-slate-400 font-semibold">{Math.round(hardware.cpu.temperatureC)}°C</span>
      {/if}
    </div>
    
    <div class="w-full h-2 bg-slate-800 rounded-full overflow-hidden">
      <div class="h-full bg-gradient-to-r from-violet-500 to-cyan-400 rounded-full transition-all duration-1000"
           style="width: {status === 'online' && hardware ? hardware.cpu.usagePercent : 0}%"></div>
    </div>
    
    <div class="mt-3 text-xs text-slate-400 truncate flex justify-between font-mono">
      <span>{status === 'online' && hardware ? `${hardware.cpu.cores} Cores` : '8 Cores'}</span>
      <span>{status === 'online' && hardware && hardware.cpu.speedGHz ? `${hardware.cpu.speedGHz.toFixed(1)} GHz` : '3.6 GHz'}</span>
    </div>
    <div class="text-[10px] text-slate-500 truncate mt-1 text-left">
      {status === 'online' && hardware ? hardware.cpu.model : 'AMD Ryzen / Intel Core'}
    </div>
  </div>

  <!-- MEMORY CARD -->
  <div class="glass rounded-2xl p-5 hover:-translate-y-1 hover:shadow-2xl hover:border-fuchsia-500/30 transition-all duration-300">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-fuchsia-500/10 flex items-center justify-center">
          <!-- standard disk icon fallback for memory -->
          <i class="fa fa-tachometer text-fuchsia-400 text-lg"></i>
        </div>
        <div>
          <div class="text-xs text-slate-400 uppercase tracking-wider font-semibold">Memory</div>
          <div class="text-lg font-bold text-white">
            {status === 'online' && hardware ? Math.round(hardware.memory.usagePercent) : 0}%
          </div>
        </div>
      </div>
      <span class="text-xs text-slate-400 font-mono">
        {#if status === 'online' && hardware}
          {hardware.memory.usedGB.toFixed(1)} / {hardware.memory.totalGB.toFixed(1)} GB
        {:else}
          0.0 / 16.0 GB
        {/if}
      </span>
    </div>
    
    <div class="w-full h-2 bg-slate-800 rounded-full overflow-hidden">
      <div class="h-full bg-gradient-to-r from-fuchsia-500 to-pink-400 rounded-full transition-all duration-1000"
           style="width: {status === 'online' && hardware ? hardware.memory.usagePercent : 0}%"></div>
    </div>
    
    <div class="mt-3 text-xs text-slate-400 text-left font-mono">
      <span>DDR Memory Status</span>
    </div>
  </div>

  <!-- GPU CARD (Simulated/Deferred) -->
  <div class="glass rounded-2xl p-5 hover:-translate-y-1 hover:shadow-2xl hover:border-emerald-500/30 transition-all duration-300">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center">
          <i class="fa fa-desktop text-emerald-400 text-lg"></i>
        </div>
        <div>
          <div class="text-xs text-slate-400 uppercase tracking-wider font-semibold">GPU</div>
          <div class="text-lg font-bold text-white">
            {status === 'online' && hardware && hardware.gpu.usagePercent ? Math.round(hardware.gpu.usagePercent) : 0}%
          </div>
        </div>
      </div>
      {#if status === 'online' && hardware && hardware.gpu.temperatureC}
        <span class="text-xs text-slate-400 font-semibold">{Math.round(hardware.gpu.temperatureC)}°C</span>
      {/if}
    </div>
    
    <div class="w-full h-2 bg-slate-800 rounded-full overflow-hidden">
      <div class="h-full bg-gradient-to-r from-emerald-500 to-teal-400 rounded-full transition-all duration-1000"
           style="width: {status === 'online' && hardware && hardware.gpu.usagePercent ? hardware.gpu.usagePercent : 0}%"></div>
    </div>
    
    <div class="mt-3 text-xs text-slate-400 truncate flex justify-between font-mono">
      <span>{status === 'online' && hardware ? hardware.gpu.model : 'Not detected'}</span>
      <span>{status === 'online' && hardware && hardware.gpu.vramGB ? `${hardware.gpu.vramGB.toFixed(1)} GB` : ''}</span>
    </div>
  </div>

  <!-- NETWORK CARD -->
  <div class="glass rounded-2xl p-5 hover:-translate-y-1 hover:shadow-2xl hover:border-sky-500/30 transition-all duration-300">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-sky-500/10 flex items-center justify-center">
          <i class="fa fa-wifi text-sky-400 text-lg"></i>
        </div>
        <div>
          <div class="text-xs text-slate-400 uppercase tracking-wider font-semibold">Network</div>
          <div class="text-lg font-bold text-white capitalize">
            {status === 'online' && hardware ? hardware.network.status : 'offline'}
          </div>
        </div>
      </div>
      {#if status === 'online' && hardware}
        <span class="text-[10px] text-slate-400 truncate font-mono max-w-[80px]" title={hardware.network.interfaceName}>
          {hardware.network.interfaceName}
        </span>
      {/if}
    </div>
    
    <div class="grid grid-cols-2 gap-2 mt-2 font-mono">
      <div class="bg-slate-900/50 border border-white/5 rounded-xl p-2 text-left">
        <p class="text-[10px] text-slate-500">↓ Download</p>
        <p class="text-sm font-semibold text-emerald-400">
          {status === 'online' && hardware ? `${hardware.network.downloadMBps.toFixed(2)} MB/s` : '0.00 MB/s'}
        </p>
      </div>
      <div class="bg-slate-900/50 border border-white/5 rounded-xl p-2 text-left">
        <p class="text-[10px] text-slate-500">↑ Upload</p>
        <p class="text-sm font-semibold text-sky-400">
          {status === 'online' && hardware ? `${hardware.network.uploadMBps.toFixed(2)} MB/s` : '0.00 MB/s'}
        </p>
      </div>
    </div>
  </div>
</div>

<!-- Storage drives section -->
<div class="glass rounded-2xl p-5 hover:border-slate-700/50 transition-all duration-300 mt-4 select-none">
  <div class="flex items-center gap-3 mb-4">
    <div class="w-10 h-10 rounded-xl bg-amber-500/10 flex items-center justify-center">
      <i class="fa fa-hdd-o text-amber-400 text-lg"></i>
    </div>
    <div class="text-left">
      <h2 class="text-base font-semibold text-white">Storage Volumes</h2>
      <p class="text-xs text-slate-500">Local drives and partitions</p>
    </div>
  </div>
  
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
    {#if status === 'online' && hardware && hardware.storage.length > 0}
      {#each hardware.storage as disk}
        <div class="bg-slate-950/40 border border-white/5 rounded-xl p-3.5 hover:border-slate-800 transition-all duration-300">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-medium text-white max-w-[150px] truncate" title={disk.name}>
              {disk.name} <span class="text-slate-500 text-xs">({disk.disk_type})</span>
            </span>
            <span class="text-xs font-semibold font-mono {disk.usage_percent > 90 ? 'text-rose-400' : 'text-slate-400'}">
              {Math.round(disk.usage_percent)}%
            </span>
          </div>
          <div class="w-full h-2 bg-slate-900 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-amber-500 to-orange-400 rounded-full transition-all duration-1000"
                 style="width: {disk.usage_percent}%"></div>
          </div>
          <div class="mt-2 text-xs text-slate-500 flex justify-between font-mono">
            <span>{disk.used_gb.toFixed(1)} GB used</span>
            <span>{disk.total_gb.toFixed(1)} GB total</span>
          </div>
        </div>
      {/each}
    {:else}
      <!-- Offline placeholder drive -->
      <div class="bg-slate-950/40 border border-white/5 rounded-xl p-3.5 select-none opacity-50">
        <div class="flex items-center justify-between mb-2">
          <span class="text-sm font-medium text-white">Local Drive (C:)</span>
          <span class="text-xs text-slate-400">0%</span>
        </div>
        <div class="w-full h-2 bg-slate-900 rounded-full"></div>
        <div class="mt-2 text-xs text-slate-500 flex justify-between">
          <span>0.0 GB used</span>
          <span>0.0 GB total</span>
        </div>
      </div>
    {/if}
  </div>
</div>
