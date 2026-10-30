<script lang="ts">
  import HelpModal from "./HelpModal.svelte";

  const COMMANDS = ["help", "status", "about", "clear", "modes", "api"];
  let lines: string[] = [
    "Frontier Terminal (local mode)",
    "WSS backend not yet implemented. Type help for local commands.",
    ""
  ];
  let input = "";
  let suggestions: string[] = [];
  let helpOpen = false;

  function push(line: string) {
    lines = [...lines, line].slice(-100);
  }

  function onInput() {
    const v = input.trim().toLowerCase();
    suggestions = v === "" ? [] : COMMANDS.filter((c) => c.startsWith(v) && c !== v);
  }

  function complete(cmd: string) {
    input = cmd;
    suggestions = [];
  }

  function send() {
    const cmd = input.trim().toLowerCase();
    push("> " + input);
    input = "";
    suggestions = [];
    if (cmd === "") return;
    if (cmd === "help") {
      push("Local commands: help, status, about, clear, modes, api");
      push("Type any command and press Enter.");
      return;
    }
    if (cmd === "status") {
      push("WSS /api/v1/terminal: not implemented");
      push("REST /api/v1/mode: available");
      push("SSE  /api/v1/events/stream: available");
      return;
    }
    if (cmd === "about") { push("rust-iot-concentrator v4.0"); return; }
    if (cmd === "clear") { lines = []; return; }
    if (cmd === "modes") { push("HARD_ONLY, SOFT_ONLY, LIFE_ONLY, HARD+SOFT, HARD+LIFE, SOFT+LIFE, ALL"); return; }
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

<div class="header">
  <h2>Terminal</h2>
  <button class="help-btn" on:click={() => (helpOpen = true)} title="Help">?</button>
</div>

<pre class="term">{lines.join("\n")}</pre>

<div class="input-wrap">
  <input bind:value={input} on:input={onInput} on:keydown={(e) => e.key === "Enter" && send()} placeholder="type help..." />
  {#if suggestions.length > 0}
    <div class="suggestions">
      {#each suggestions as s}
        <button on:click={() => complete(s)}>{s}</button>
      {/each}
    </div>
  {/if}
</div>

<HelpModal bind:open={helpOpen} section="Terminal" />

<style>
  .header { display: flex; justify-content: space-between; align-items: center; }
  .help-btn { background: #334155; border: 1px solid #475569; color: #e2e8f0; width: 28px; height: 28px; border-radius: 50%; cursor: pointer; font-weight: bold; }
  .term { background: #020617; padding: 1rem; border-radius: 8px; min-height: 260px; white-space: pre-wrap; font-family: monospace; font-size: 0.9rem; }
  .input-wrap { position: relative; margin-top: 0.5rem; }
  input { width: 100%; padding: 0.5rem; background: #1e293b; color: #e2e8f0; border: 1px solid #334155; border-radius: 6px; }
  .suggestions { position: absolute; bottom: 100%; left: 0; background: #1e293b; border: 1px solid #334155; border-radius: 6px; overflow: hidden; }
  .suggestions button { display: block; width: 100%; text-align: left; padding: 0.25rem 0.75rem; background: transparent; border: none; color: #e2e8f0; cursor: pointer; }
  .suggestions button:hover { background: #334155; }
</style>
