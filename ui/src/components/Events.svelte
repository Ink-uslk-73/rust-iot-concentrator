<script lang="ts">
  import { onMount } from "svelte";
  import { getEvents, ackEvent } from "../api/rest";

  let events: any[] = [];
  let error = "";

  async function load() {
    try {
      events = await getEvents();
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function ack(id: string) {
    try {
      await ackEvent(id, "tablet");
      await load();
    } catch (e) {
      error = String(e);
    }
  }

  onMount(load);
</script>

<h2>Events (ATT&CK)</h2>
{#if error}<p class="err">{error}</p>{/if}
<table>
  <thead>
    <tr><th>TS</th><th>Device</th><th>Type</th><th>Severity</th><th>Technique</th><th>Ack</th></tr>
  </thead>
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

<style>
  table { width: 100%; border-collapse: collapse; margin-top: 1rem; }
  th, td { text-align: left; padding: 0.5rem; border-bottom: 1px solid #334155; font-size: 0.9rem; }
  .critical { color: #ef4444; }
  .high { color: #f59e0b; }
  .medium { color: #eab308; }
  .err { color: #ef4444; }
</style>
