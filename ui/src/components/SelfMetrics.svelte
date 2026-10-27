<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  let cpuAvg = 0;
  let memUsed = 0;
  let numaForeign = 0;
  let cpuTemp = 0;
  let timer: ReturnType<typeof setInterval>;

  async function refresh() {
    try {
      const r = await fetch("/api/v1/metrics/self");
      if (r.ok) {
        const m = await r.json();
        cpuAvg = m["self.cpu.avg"] ?? 0;
        memUsed = m["self.mem.used_ratio"] ?? 0;
        numaForeign = m["self.numa.foreign"] ?? 0;
        cpuTemp = m["self.temp.cpu"] ?? 0;
        console.log("updated:", cpuAvg, memUsed);
      }
    } catch (e) { console.error(e); }
  }

  onMount(() => { refresh(); timer = setInterval(refresh, 2000); });
  onDestroy(() => clearInterval(timer));
</script>

<h2>Self Metrics</h2>
<div class="grid">
  <div class="card"><div class="label">CPU avg</div><div class="value">{(cpuAvg * 100).toFixed(1)}%</div></div>
  <div class="card"><div class="label">Mem used</div><div class="value">{(memUsed * 100).toFixed(1)}%</div></div>
  <div class="card"><div class="label">NUMA foreign</div><div class="value">{numaForeign}</div></div>
  <div class="card"><div class="label">CPU temp</div><div class="value">{cpuTemp}C</div></div>
</div>

<style>
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 1rem; margin-top: 1rem; }
  .card { background: #1e293b; padding: 1rem; border-radius: 8px; }
  .label { font-size: 0.85rem; color: #94a3b8; }
  .value { font-size: 1.4rem; font-weight: 600; margin-top: 0.5rem; }
</style>
