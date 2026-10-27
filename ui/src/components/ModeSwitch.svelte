<script lang="ts">
  import { onMount } from "svelte";
  import { getMode, setMode } from "../api/rest";

  const MODES = [
    "HARD_ONLY", "SOFT_ONLY", "LIFE_ONLY",
    "HARD+SOFT", "HARD+LIFE", "SOFT+LIFE", "ALL"
  ];

  let current = "";
  let error = "";
  let busy = false;

  async function load() {
    try {
      const m = await getMode();
      current = m.mode;
    } catch (e) {
      error = String(e);
    }
  }

  async function change(mode: string) {
    if (busy) return;
    busy = true;
    try {
      const m = await setMode(mode, "tablet");
      current = m.mode;
      error = "";
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  onMount(load);
</script>

<h2>Mode Switch</h2>
<p>Current: <strong>{current || "..."}</strong></p>
{#if error}<p class="err">{error}</p>{/if}

<div class="modes">
  {#each MODES as m}
    <button
      class:active={m === current}
      disabled={busy}
      on:click={() => change(m)}
    >
      {m}
    </button>
  {/each}
</div>

<style>
  .modes { display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 0.5rem; margin-top: 1rem; }
  .modes button { background: #1e293b; border: 1px solid #334155; color: #e2e8f0; padding: 0.75rem; border-radius: 6px; cursor: pointer; }
  .modes button.active { background: #2563eb; border-color: #3b82f6; }
  .modes button:disabled { opacity: 0.5; cursor: wait; }
  .err { color: #ef4444; }
</style>
