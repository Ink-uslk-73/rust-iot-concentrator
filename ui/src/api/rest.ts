const BASE = "/api/v1";

export async function getOverhead(): Promise<any> {
  const r = await fetch(`${BASE}/metrics/overhead`);
  if (!r.ok) throw new Error(`HTTP ${r.status}`);
  return r.json();
}

export async function getEvents(): Promise<any[]> {
  const r = await fetch(`${BASE}/events`);
  if (!r.ok) throw new Error(`HTTP ${r.status}`);
  return r.json();
}

export async function getBaselineSummary(deviceId: string): Promise<any> {
  const r = await fetch(`${BASE}/devices/${deviceId}/baseline/summary`);
  if (!r.ok) throw new Error(`HTTP ${r.status}`);
  return r.json();
}

export async function getMode(): Promise<any> {
  const r = await fetch(`${BASE}/mode`);
  if (!r.ok) throw new Error(`HTTP ${r.status}`);
  return r.json();
}

export async function setMode(mode: string, operator: string): Promise<any> {
  const r = await fetch(`${BASE}/mode`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ mode, operator })
  });
  if (!r.ok) throw new Error(`HTTP ${r.status}`);
  return r.json();
}

export async function ackEvent(eventId: string, by: string): Promise<void> {
  const r = await fetch(`${BASE}/events/ack`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ event_id: eventId, by })
  });
  if (!r.ok) throw new Error(`HTTP ${r.status}`);
}
