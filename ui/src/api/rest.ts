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
