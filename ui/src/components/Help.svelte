<h2>Help</h2>

<section>
  <h3>Dashboard</h3>
  <p>Overhead metrics: HARD latency, API latency, SSE jitter, bus buffer, bus drops. Status: ok / degraded / critical.</p>
</section>

<section>
  <h3>Mode Switch</h3>
  <p>7 modes: HARD_ONLY, SOFT_ONLY, LIFE_ONLY, HARD+SOFT, HARD+LIFE, SOFT+LIFE, ALL.</p>
  <p>POST /api/v1/mode with body: mode, operator.</p>
</section>

<section>
  <h3>Telemetry</h3>
  <p>Live events (SSE /api/v1/events/stream).</p>
</section>

<section>
  <h3>Events</h3>
  <p>ATT&CK for ICS events. Ack via POST /api/v1/events/ack.</p>
</section>

<section>
  <h3>Terminal</h3>
  <p>Local commands: help, status, about, clear, modes, api.</p>
</section>
