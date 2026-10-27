<script lang="ts">
  import { onMount } from "svelte";
  import { getOverhead } from "../api/rest";
  let overhead: any = null;
  onMount(async () => {
    try { overhead = await getOverhead(); } catch (e) { console.error(e); }
  });
</script>

<h2>Dashboard</h2>
{#if overhead}
  <pre>{JSON.stringify(overhead, null, 2)}</pre>
{:else}
  <p>loading...</p>
{/if}
