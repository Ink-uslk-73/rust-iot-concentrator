<script lang="ts">
  export let open = false;
  export let section = "";

  function close() {
    open = false;
  }
</script>

{#if open}
<div class="overlay" on:click={close} on:keydown={(e) => e.key === "Escape" && close()} role="presentation">
  <div class="modal" on:click|stopPropagation role="dialog" aria-modal="true">
    <header>
      <h2>Help {section ? " " + section : ""}</h2>
      <button on:click={close} aria-label="close">x</button>
    </header>
    <div class="body">
      {#if section === "Dashboard" || !section}
        <h3>Dashboard</h3>
        <p>Overhead metrics. Thresholds:</p>
        <ul>
          <li>HARD latency: &lt; 1 ms ok, 1-5 ms degraded, &gt; 5 ms critical</li>
          <li>API latency: &lt; 1 ms ok, &gt; 1 ms degraded</li>
          <li>Bus buffer: &lt; 70% ok, 70-90% degraded, &gt; 90% critical</li>
          <li>Bus drops: 0 ok, &gt; 0 critical</li>
          <li>SSE jitter: &lt; 10 ms ok, &gt; 10 ms degraded</li>
        </ul>
      {/if}
      {#if section === "Mode Switch" || !section}
        <h3>Mode Switch</h3>
        <p>7 modes: HARD_ONLY, SOFT_ONLY, LIFE_ONLY, HARD+SOFT, HARD+LIFE, SOFT+LIFE, ALL.</p>
        <p>POST /api/v1/mode with body: mode, operator.</p>
      {/if}
      {#if section === "Terminal" || !section}
        <h3>Terminal</h3>
        <p>Local commands: help, status, about, clear, modes, api.</p>
        <p>Type any command and press Enter. Autocomplete: start typing.</p>
      {/if}
      {#if section === "Events" || !section}
        <h3>Events</h3>
        <p>ATT&CK for ICS events. Ack via POST /api/v1/events/ack.</p>
      {/if}
      {#if section === "Telemetry" || !section}
        <h3>Telemetry</h3>
        <p>Live events (SSE /api/v1/events/stream).</p>
      {/if}
    </div>
  </div>
</div>
{/if}

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.6); display: flex; align-items: center; justify-content: center; z-index: 200; }
  .modal { background: #1e293b; color: #e2e8f0; border-radius: 8px; max-width: 640px; width: 90%; max-height: 80vh; overflow: auto; padding: 1rem; }
  header { display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #334155; padding-bottom: 0.5rem; margin-bottom: 1rem; }
  header h2 { margin: 0; font-size: 1.1rem; }
  header button { background: transparent; border: 1px solid #334155; color: #e2e8f0; border-radius: 4px; padding: 0.25rem 0.5rem; cursor: pointer; }
  h3 { color: #93c5fd; margin-bottom: 0.5rem; }
  ul { margin-left: 1rem; }
</style>
