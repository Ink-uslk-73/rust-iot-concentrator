<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getOverhead } from "../api/rest";
  import Tooltip from "./Tooltip.svelte";
  import HelpModal from "./HelpModal.svelte";

  let overhead: any = null;
  let error = "";
  let timer: ReturnType<typeof setInterval>;
  let helpOpen = false;

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

  function cls(v: number, ok: number, deg: number): string {
    if (v === undefined || v === null) return "";
    if (v > deg) return "crit";
    if (v > ok) return "warn";
    return "ok";
  }
</script>

<div class="header">
  <h2>Dashboard</h2>
  <button class="help-btn" on:click={() => (helpOpen = true)} title="Help">?</button>
</div>

{#if error}
  <p class="err">{error}</p>
{:else if overhead}
  <div class="grid">
    <div class="card">
      <div class="label">Status <Tooltip text="ok / degraded / critical">{overhead.status}</Tooltip></div>
      <div class="value {overhead.status === 'ok' ? 'ok' : overhead.status === 'degraded' ? 'warn' : 'crit'}">{overhead.status}</div>
    </div>
    <div class="card">
      <div class="label">HARD latency <Tooltip text="&lt; 1 ms ok, 1-5 ms degraded, &gt; 5 ms critical" /></div>
      <div class="value {cls(overhead.hard_latency_us, 1000, 5000)}">{overhead.hard_latency_us} us</div>
    </div>
    <div class="card">
      <div class="label">API latency <Tooltip text="&lt; 1 ms ok, &gt; 1 ms degraded" /></div>
      <div class="value {cls(overhead.api_latency_us, 1000, 5000)}">{overhead.api_latency_us} us</div>
    </div>
    <div class="card">
      <div class="label">Bus buffer <Tooltip text="&lt; 70% ok, 70-90% degraded, &gt; 90% critical" /></div>
      <div class="value {cls(overhead.bus_buffer_pct, 70, 90)}">{overhead.bus_buffer_pct}%</div>
    </div>
    <div class="card">
      <div class="label">Bus drops <Tooltip text="0 ok, &gt; 0 critical" /></div>
      <div class="value {overhead.bus_drops > 0 ? 'crit' : 'ok'}">{overhead.bus_drops}</div>
    </div>
    <div class="card">
      <div class="label">SSE jitter <Tooltip text="&lt; 10 ms ok, &gt; 10 ms degraded" /></div>
      <div class="value {cls(overhead.sse_jitter_ms, 10, 50)}">{overhead.sse_jitter_ms} ms</div>
    </div>
  </div>
{:else}
  <p>loading...</p>
{/if}

<HelpModal bind:open={helpOpen} section="Dashboard" />

<style>
  .header { display: flex; justify-content: space-between; align-items: center; }
  .help-btn { background: #334155; border: 1px solid #475569; color: #e2e8f0; width: 28px; height: 28px; border-radius: 50%; cursor: pointer; font-weight: bold; }
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 1rem; margin-top: 1rem; }
  .card { background: #1e293b; padding: 1rem; border-radius: 8px; }
  .label { font-size: 0.85rem; color: #94a3b8; }
  .value { font-size: 1.4rem; font-weight: 600; margin-top: 0.5rem; }
  .value.ok { color: #22c55e; }
  .value.warn { color: #f59e0b; }
  .value.crit { color: #ef4444; }
  .err { color: #ef4444; }
</style>
