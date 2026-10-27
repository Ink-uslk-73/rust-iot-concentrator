<script lang="ts">
  import { onMount } from "svelte";
  import { getMode, setMode } from "../api/rest";
  import Tooltip from "./Tooltip.svelte";
  import HelpModal from "./HelpModal.svelte";

  const MODES = [
    { name: "HARD_ONLY", h: true, s: false, l: false, desc: "Only HARD (10 ms SLA)" },
    { name: "SOFT_ONLY", h: false, s: true, l: false, desc: "Only SOFT (100 ms SLA)" },
    { name: "LIFE_ONLY", h: false, s: false, l: true, desc: "Only LIFE (1000 ms SLA)" },
    { name: "HARD+SOFT", h: true, s: true, l: false, desc: "HARD + SOFT, no LIFE" },
    { name: "HARD+LIFE", h: true, s: false, l: true, desc: "HARD + LIFE, no SOFT" },
    { name: "SOFT+LIFE", h: false, s: true, l: true, desc: "SOFT + LIFE, no HARD" },
    { name: "ALL", h: true, s: true, l: true, desc: "All domains (hybrid)" }
  ];

  let current = "";
  let error = "";
  let busy = false;
  let helpOpen = false;

  async function load() {
    try { const m = await getMode(); current = m.mode; } catch (e) { error = String(e); }
  }
  async function change(mode: string) {
    if (busy) return;
    busy = true;
    try { const m = await setMode(mode, "tablet"); current = m.mode; error = ""; }
    catch (e) { error = String(e); }
    finally { busy = false; }
  }
  onMount(load);
</script>

<div class="header">
  <h2>Mode Switch</h2>
  <button class="help-btn" on:click={() => (helpOpen = true)} title="Help">?</button>
</div>

<p>Current: <strong>{current || "..."}</strong></p>
{#if error}<p class="err">{error}</p>{/if}

<div class="modes">
  {#each MODES as m}
    <Tooltip text={m.desc}>
      <button class:active={m.name === current} disabled={busy} on:click={() => change(m.name)}>
        <div class="name">{m.name}</div>
        <div class="domains">
          <span class:on={m.h} class:off={!m.h}>H</span>
          <span class:on={m.s} class:off={!m.s}>S</span>
          <span class:on={m.l} class:off={!m.l}>L</span>
        </div>
      </button>
    </Tooltip>
  {/each}
</div>

<HelpModal bind:open={helpOpen} section="Mode Switch" />

<style>
  .header { display: flex; justify-content: space-between; align-items: center; }
  .help-btn { background: #334155; border: 1px solid #475569; color: #e2e8f0; width: 28px; height: 28px; border-radius: 50%; cursor: pointer; font-weight: bold; }
  .modes { display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: 0.5rem; margin-top: 1rem; }
  .modes button { background: #1e293b; border: 1px solid #334155; color: #e2e8f0; padding: 0.75rem; border-radius: 6px; cursor: pointer; text-align: left; }
  .modes button.active { background: #2563eb; border-color: #3b82f6; }
  .modes button:disabled { opacity: 0.5; cursor: wait; }
  .name { font-weight: 600; margin-bottom: 0.25rem; }
  .domains span { display: inline-block; width: 20px; height: 20px; line-height: 20px; text-align: center; border-radius: 3px; margin-right: 4px; font-size: 0.75rem; }
  .on { background: #22c55e; color: #052e16; }
  .off { background: #334155; color: #64748b; }
  .err { color: #ef4444; }
</style>
