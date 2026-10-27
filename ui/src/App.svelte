<script lang="ts">
  import { onMount } from "svelte";
  import Dashboard from "./components/Dashboard.svelte";
  import ModeSwitch from "./components/ModeSwitch.svelte";
  import ConfigMode from "./components/ConfigMode.svelte";
  import SelfMetrics from "./components/SelfMetrics.svelte";
  import Telemetry from "./components/Telemetry.svelte";
  import Events from "./components/Events.svelte";
  import Terminal from "./components/Terminal.svelte";
  import Help from "./components/Help.svelte";
  import Clock from "./components/Clock.svelte";

  let page: "dashboard" | "telemetry" | "events" | "terminal" | "help" = "dashboard";
  let connected = false;

  onMount(() => {
    connected = true;
  });
</script>

<div class="layout">
  <aside class="sidebar">
    <h1>Concentrator</h1>
    <nav>
      <button class:active={page === "dashboard"} on:click={() => (page = "dashboard")}>Dashboard</button>
      <button class:active={page === "telemetry"} on:click={() => (page = "telemetry")}>Telemetry</button>
      <button class:active={page === "events"} on:click={() => (page = "events")}>Events</button>
      <button class:active={page === "terminal"} on:click={() => (page = "terminal")}>Terminal</button>
      <button class:active={page === "help"} on:click={() => (page = "help")}>Help</button>
    </nav>
    <div class="status" class:ok={connected} class:err={!connected}>
      {connected ? "connected" : "disconnected"}
    </div>
  </aside>

  <div class="content">
    <header class="topbar">
      <div class="topbar-left">{page}</div>
      <div class="topbar-right"><Clock /></div>
    </header>

    <main class="main">
      {#if page === "dashboard"}
        <Dashboard />
        <ModeSwitch />
        <ConfigMode />
        <SelfMetrics />
      {:else if page === "telemetry"}
        <Telemetry />
      {:else if page === "events"}
        <Events />
      {:else if page === "terminal"}
        <Terminal />
      {:else if page === "help"}
        <Help />
      {/if}
    </main>
  </div>
</div>

<style>
  .layout { display: flex; height: 100vh; }
  .sidebar { width: 220px; background: #1e293b; padding: 1rem; display: flex; flex-direction: column; gap: 1rem; }
  .sidebar h1 { font-size: 1.1rem; margin: 0; }
  .sidebar nav { display: flex; flex-direction: column; gap: 0.5rem; }
  .sidebar button { background: transparent; border: 1px solid #334155; color: #e2e8f0; padding: 0.5rem; text-align: left; cursor: pointer; border-radius: 4px; }
  .sidebar button.active { background: #334155; }
  .status { margin-top: auto; font-size: 0.85rem; padding: 0.5rem; border-radius: 4px; text-align: center; }
  .status.ok { background: #14532d; }
  .status.err { background: #7f1d1d; }
  .content { flex: 1; display: flex; flex-direction: column; }
  .topbar { display: flex; justify-content: space-between; align-items: center; padding: 0.5rem 1rem; background: #1e293b; border-bottom: 1px solid #334155; }
  .topbar-left { font-size: 0.9rem; color: #94a3b8; text-transform: capitalize; }
  .topbar-right { display: flex; justify-content: flex-end; }
  .main { flex: 1; padding: 1rem; overflow: auto; }
</style>
