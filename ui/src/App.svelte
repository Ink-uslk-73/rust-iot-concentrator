<script lang="ts">
  import { onMount } from "svelte";
  import Dashboard from "./components/Dashboard.svelte";
  import ModeSwitch from "./components/ModeSwitch.svelte";
  import Telemetry from "./components/Telemetry.svelte";
  import Events from "./components/Events.svelte";
  import Terminal from "./components/Terminal.svelte";

  let page: "dashboard" | "telemetry" | "events" | "terminal" = "dashboard";
  let connected = false;

  onMount(() => {
    // TODO: WSS connect (mTLS) to /api/v1/events/stream
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
    </nav>
    <div class="status" class:ok={connected} class:err={!connected}>
      {connected ? "connected" : "disconnected"}
    </div>
  </aside>

  <main class="main">
    {#if page === "dashboard"}
      <Dashboard />
      <ModeSwitch />
    {:else if page === "telemetry"}
      <Telemetry />
    {:else if page === "events"}
      <Events />
    {:else if page === "terminal"}
      <Terminal />
    {/if}
  </main>
</div>
