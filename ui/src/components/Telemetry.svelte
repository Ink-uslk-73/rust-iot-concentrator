<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { connectEvents } from "../api/ws";
  import HelpModal from "./HelpModal.svelte";

  let items: any[] = [];
  let stop: (() => void) | null = null;
  let helpOpen = false;

  onMount(() => {
    stop = connectEvents((data) => {
      if (Array.isArray(data)) items = [...data, ...items].slice(0, 50);
    });
  });
  onDestroy(() => stop?.());
</script>

<div class="header">
  <h2>Telemetry</h2>
  <button class="help-btn" on:click={() => (helpOpen = true)} title="Help">?</button>
</div>

{#if items.length === 0}
  <p class="empty">No telemetry yet. Waiting for SSE events...</p>
{:else}
  <ul>
    {#each items as e}
      <li><strong>{e.severity}</strong> {e.event_type}  {e.device_id} {#if e.technique_id}<code>{e.technique_id}</code>{/if}</li>
    {/each}
  </ul>
{/if}

<HelpModal bind:open={helpOpen} section="Telemetry" />

<style>
  .header { display: flex; justify-content: space-between; align-items: center; }
  .help-btn { background: #334155; border: 1px solid #475569; color: #e2e8f0; width: 28px; height: 28px; border-radius: 50%; cursor: pointer; font-weight: bold; }
  .empty { color: #94a3b8; font-style: italic; }
  code { background: #1e293b; padding: 0.1rem 0.3rem; border-radius: 3px; }
</style>
