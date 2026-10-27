<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { connectEvents } from "../api/ws";

  let items: any[] = [];
  let stop: (() => void) | null = null;

  onMount(() => {
    stop = connectEvents((data) => {
      if (Array.isArray(data)) {
        items = [...data, ...items].slice(0, 50);
      }
    });
  });

  onDestroy(() => stop?.());
</script>

<h2>Telemetry</h2>
<p>Live events (SSE): {items.length}</p>
<ul>
  {#each items as e}
    <li>
      <strong>{e.severity}</strong> {e.event_type}  {e.device_id}
      {#if e.technique_id}<code>{e.technique_id}</code>{/if}
    </li>
  {/each}
</ul>
