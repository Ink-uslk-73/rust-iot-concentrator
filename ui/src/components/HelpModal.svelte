<script lang="ts">
  export let open = false;
  export let section = "";
  function close() { open = false; }
  function onKey(e: KeyboardEvent) { if (e.key === "Escape") close(); }
</script>

{#if open}
<div class="overlay" role="presentation" on:click={close} on:keydown={onKey}>
  <div class="modal" role="dialog" aria-modal="true"
    aria-label={section ? "Help: " + section : "Help"} tabindex="-1"
    on:click|stopPropagation>
    <header>
      <h2>Help {section ? "- " + section : ""}</h2>
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
  .overlay {
    position: fixed; inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex; align-items: center; justify-content: center;
    z-index: 200;
  }
  .modal {
    background: #f8fafc;
    color: #0f172a;
    border-radius: 8px;
    max-width: 640px; width: 90%; max-height: 80vh;
    overflow: auto; padding: 1rem; outline: none;
    border: 1px solid #cbd5e1;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.35);
    line-height: 1.5;
  }
  header {
    display: flex; justify-content: space-between; align-items: center;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 0.5rem; margin-bottom: 1rem;
  }
  header h2 { margin: 0; font-size: 1.15rem; font-weight: 600; color: #0f172a; }
  header button {
    background: #e2e8f0; border: 1px solid #94a3b8; color: #0f172a;
    border-radius: 4px; padding: 0.25rem 0.6rem; cursor: pointer; font-weight: 600;
  }
  header button:hover { background: #cbd5e1; }
  h3 { color: #1e40af; margin: 0.75rem 0 0.5rem 0; font-size: 1rem; font-weight: 600; }
  p, li { color: #1e293b; line-height: 1.5; }
  ul { margin-left: 1.25rem; }
</style>
