<script lang="ts">
  // TODO: xterm.js + WSS /api/v1/terminal (backend not yet implemented)
  let output = "Terminal (WSS backend not yet implemented)\n";
  let input = "";

  function send() {
    output += "> " + input + "\n(not connected)\n";
    input = "";
  }
</script>

<h2>Terminal</h2>
<pre class="term">{output}</pre>
<input bind:value={input} on:keydown={(e) => e.key === "Enter" && send()} placeholder="command..." />

<style>
  .term { background: #020617; padding: 1rem; border-radius: 8px; min-height: 200px; white-space: pre-wrap; font-family: monospace; }
  input { margin-top: 0.5rem; width: 100%; padding: 0.5rem; background: #1e293b; color: #e2e8f0; border: 1px solid #334155; border-radius: 6px; }
</style>
