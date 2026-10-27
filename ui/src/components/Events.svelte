<script lang="ts">
  import { onMount } from "svelte";
  import { getEvents, ackEvent } from "../api/rest";
  import HelpModal from "./HelpModal.svelte";

  let events: any[] = [];
  let error = "";
  let helpOpen = false;

  async function load() {
    try { events = await getEvents(); error = ""; } catch (e) { error = String(e); }
  }
  async function ack(id: string) {
    try { await ackEvent(id, "tablet"); await load(); } catch (e) { error = String(e); }
  }
  onMount(load);
</script>

<div class="header">
  <h2>Events (ATT&CK)</h2>
  <button class="help-btn" on:click={() => (helpOpen = true)} title="Help">?</button>
</div>

{#if error}<p class="err">{error}</p>{/if}

{#if events.length === 0}
  <p class="empty">No events yet. SSE connected, waiting for ATT&CK events.</p>
{:else}
  <table>
    <thead><tr><th>TS</th><th>Device</th><th>Type</th><th>Severity</th><th>Technique</th><th>Ack</th></tr></thead>
    <tbody>
      {#each events as e}
        <tr>
          <td>{e.ts}</td>
          <td>{e.device_id}</td>
          <td>{e.event_type}</td>
          <td class={e.severity}>{e.severity}</td>
          <td>{e.technique_id || "-"}</td>
          <td>{#if !e.acked}<button on:click={() => ack(e.event_id)}>ack</button>{:else}ok{/if}</td>
        </tr>
      {/each}
    </tbody>
  </table>
{/if}

<HelpModal bind:open={helpOpen} section="Events" />

<style>
  .header { display: flex; justify-content: space-between; align-items: center; }
  .help-btn { background: #334155; border: 1px solid #475569; color: #e2e8f0; width: 28px; height: 28px; border-radius: 50%; cursor: pointer; font-weight: bold; }
  table { width: 100%; border-collapse: collapse; margin-top: 1rem; }
  th, td { text-align: left; padding: 0.5rem; border-bottom: 1px solid #334155; font-size: 0.9rem; }
  .critical { color: #ef4444; }
  .high { color: #f59e0b; }
  .medium { color: #eab308; }
  .empty { color: #94a3b8; font-style: italic; }
  .err { color: #ef4444; }
</style>
