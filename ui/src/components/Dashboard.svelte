<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getOverhead } from "../api/rest";

  let overhead: any = null;
  let error = "";
  let timer: ReturnType<typeof setInterval>;

  async function refresh() {
    try {
      overhead = await getOverhead();
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  onMount(() => {
    refresh();
    timer = setInterval(refresh, 2000);
  });

  onDestroy(() => clearInterval(timer));

  $: statusClass = overhead?.status === "ok"
    ? "ok"
    : overhead?.status === "degraded"
      ? "warn"
      : overhead?.status === "critical"
        ? "crit"
        : "";
</script>

<h2>Dashboard</h2>

{#if error}
  <p class="err">{error}</p>
{:else if overhead}
  <div class="grid">
    <div class="card">
      <div class="label">Status</div>
      <div class="value {statusClass}">{overhead.status}</div>
    </div>
    <div class="card">
      <div class="label">HARD latency</div>
      <div class="value">{overhead.hard_latency_us} us</div>
    </div>
    <div class="card">
      <div class="label">API latency</div>
      <div class="value">{overhead.api_latency_us} us</div>
    </div>
    <div class="card">
      <div class="label">Bus buffer</div>
      <div class="value">{overhead.bus_buffer_pct}%</div>
    </div>
    <div class="card">
      <div class="label">Bus drops</div>
      <div class="value">{overhead.bus_drops}</div>
    </div>
    <div class="card">
      <div class="label">SSE jitter</div>
      <div class="value">{overhead.sse_jitter_ms} ms</div>
    </div>
  </div>
{:else}
  <p>loading...</p>
{/if}

<style>
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 1rem; margin-top: 1rem; }
  .card { background: #1e293b; padding: 1rem; border-radius: 8px; }
  .label { font-size: 0.85rem; color: #94a3b8; }
  .value { font-size: 1.4rem; font-weight: 600; margin-top: 0.5rem; }
  .value.ok { color: #22c55e; }
  .value.warn { color: #f59e0b; }
  .value.crit { color: #ef4444; }
  .err { color: #ef4444; }
</style>
