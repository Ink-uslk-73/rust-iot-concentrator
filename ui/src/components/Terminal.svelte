<script lang="ts">
  let lines: string[] = [
    "Concentrator Terminal (local mode)",
    "WSS backend not yet implemented. Type help for local commands.",
    ""
  ];
  let input = "";

  function push(line: string) {
    lines = [...lines, line].slice(-100);
  }

  function send() {
    const cmd = input.trim().toLowerCase();
    push("> " + input);
    input = "";
    if (cmd === "") return;
    if (cmd === "help") {
      push("Local commands:");
      push("  help    - this message");
      push("  status  - backend status");
      push("  about   - version, API links");
      push("  clear   - clear output");
      push("  modes   - list 7 modes");
      push("  api     - list API endpoints");
      return;
    }
    if (cmd === "status") {
      push("WSS /api/v1/terminal: not implemented");
      push("REST /api/v1/mode: available");
      push("SSE  /api/v1/events/stream: available");
      return;
    }
    if (cmd === "about") {
      push("rust-iot-concentrator v4.0");
      push("API: /api/v1/  |  SSE: /api/v1/events/stream");
      return;
    }
    if (cmd === "clear") {
      lines = [];
      return;
    }
    if (cmd === "modes") {
      push("HARD_ONLY, SOFT_ONLY, LIFE_ONLY, HARD+SOFT, HARD+LIFE, SOFT+LIFE, ALL");
      return;
    }
    if (cmd === "api") {
      push("GET  /api/v1/mode");
      push("POST /api/v1/mode {mode, operator}");
      push("GET  /api/v1/metrics/overhead");
      push("GET  /api/v1/events");
      push("GET  /api/v1/events/stream (SSE)");
      push("POST /api/v1/events/ack {event_id, by}");
      return;
    }
    push("unknown command: " + cmd + " (type help)");
  }
</script>

<h2>Terminal</h2>
<pre class="term">{lines.join("\n")}</pre>
<input bind:value={input} on:keydown={(e) => e.key === "Enter" && send()} placeholder="type help..." />

<style>
  .term { background: #020617; padding: 1rem; border-radius: 8px; min-height: 260px; white-space: pre-wrap; font-family: monospace; font-size: 0.9rem; }
  input { margin-top: 0.5rem; width: 100%; padding: 0.5rem; background: #1e293b; color: #e2e8f0; border: 1px solid #334155; border-radius: 6px; }
</style>
