<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  let diag: any[] = [];
  let satellites: any[] = [];
  let cpuAvg = 0, memUsed = 0;
  let timer: ReturnType<typeof setInterval>;
  let open = true;

  async function refresh() {
    try {
      const [d, m, s] = await Promise.all([
        fetch("/api/v1/events?severity=critical&limit=5").then(r => r.ok ? r.json() : []),
        fetch("/api/v1/metrics/self").then(r => r.ok ? r.json() : {}),
        fetch("/api/v1/satellites").then(r => r.ok ? r.json() : []),
      ]);
      diag = Array.isArray(d) ? d : [];
      cpuAvg = m["self.cpu.avg"] ?? 0;
      memUsed = m["self.mem.used_ratio"] ?? 0;
      satellites = Array.isArray(s) ? s : [];
    } catch (e) { console.error(e); }
  }

  onMount(() => { refresh(); timer = setInterval(refresh, 2000); });
  onDestroy(() => clearInterval(timer));
</script>

<div class="diagbar">
  <button class="toggle" on:click={() => (open = !open)}>
    {open ? "v" : "^"} diag ({diag.length}) | CPU {(cpuAvg*100).toFixed(0)}% | Mem {(memUsed*100).toFixed(0)}%
  </button>
  {#if open}
    <div class="content">
      {#each satellites as s}
        <span class="sat {s.state}">{s.name}: {s.state}</span>
      {/each}
      {#each diag as d}
        <span class="diag {d.severity}">{d.event_type}</span>
      {/each}
    </div>
  {/if}
</div>

<style>
  .diagbar { background: #1e293b; border-top: 1px solid #334155; font-size: 0.8rem; z-index: 50; }
  .toggle { background: transparent; border: none; color: #e2e8f0; padding: 0.25rem 0.5rem; cursor: pointer; }
  .content { display: flex; gap: 0.75rem; padding: 0.25rem 0.5rem; overflow-x: auto; }
  .sat { padding: 0 0.5rem; border-radius: 3px; }
  .sat.running { background: #14532d; }
  .sat.stopped { background: #7f1d1d; }
  .sat.unknown { background: #334155; }
  .diag { padding: 0 0.5rem; border-radius: 3px; }
  .diag.critical { background: #7f1d1d; }
  .diag.high { background: #78350f; }
</style>
