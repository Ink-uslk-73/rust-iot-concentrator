<script lang="ts">
  import { onMount } from "svelte";

  let mode = "";
  let configPath = "";
  let registersPath = "";
  let error = "";
  let busy = false;

  async function load() {
    try {
      const r = await fetch("/api/v1/config/mode");
      if (!r.ok) throw new Error("HTTP " + r.status);
      const d = await r.json();
      mode = d.mode;
      configPath = d.config_path;
      registersPath = d.registers_path;
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function change(newMode: string) {
    if (busy) return;
    busy = true;
    try {
      const r = await fetch("/api/v1/config/mode", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ mode: newMode })
      });
      if (!r.ok) throw new Error("HTTP " + r.status);
      const d = await r.json();
      mode = d.mode;
      configPath = d.config_path;
      registersPath = d.registers_path;
      error = "";
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  onMount(load);
</script>

<div class="header">
  <h2>Config Mode</h2>
</div>

<p>Current: <strong>{mode || "..."}</strong></p>
{#if error}<p class="err">{error}</p>{/if}

<div class="modes">
  <button class:active={mode === "emulation"} disabled={busy} on:click={() => change("emulation")}>
    emulation
  </button>
  <button class:active={mode === "production"} disabled={busy} on:click={() => change("production")}>
    production
  </button>
</div>

<div class="paths">
  <div><span class="label">config:</span> <code>{configPath || "..."}</code></div>
  <div><span class="label">registers:</span> <code>{registersPath || "..."}</code></div>
</div>

<style>
  .header { display: flex; justify-content: space-between; align-items: center; }
  .modes { display: flex; gap: 0.5rem; margin-top: 1rem; }
  .modes button { background: #1e293b; border: 1px solid #334155; color: #e2e8f0; padding: 0.5rem 1rem; border-radius: 6px; cursor: pointer; }
  .modes button.active { background: #2563eb; border-color: #3b82f6; }
  .modes button:disabled { opacity: 0.5; cursor: wait; }
  .paths { margin-top: 1rem; font-size: 0.85rem; color: #94a3b8; }
  .paths code { background: #1e293b; padding: 0.1rem 0.3rem; border-radius: 3px; }
  .label { color: #64748b; }
  .err { color: #ef4444; }
</style>
