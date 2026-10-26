# API - baseline and events

REST + SSE for tablet (UI) and SOC (SIEM).

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET  | /api/v1/devices/{id}/baseline          | Latest baseline |
| GET  | /api/v1/devices/{id}/baseline/summary  | Summary |
| GET  | /api/v1/devices/{id}/baseline/diff     | Drift (stub) |
| POST | /api/v1/devices/{id}/baseline/capture  | Capture (needs approval_ticket) |
| GET  | /api/v1/devices/{id}/baseline/history  | History |
| GET  | /api/v1/events                         | List (filters: device_id, severity, technique, since, limit) |
| GET  | /api/v1/events/stream                  | SSE stream |
| GET  | /api/v1/events/{event_id}              | One event |
| POST | /api/v1/events/ack                     | Ack |

## Storage

- PostgreSQL is primary. Migrations auto-run on connect.
- JSON files under `baselines/` and `events/` are hot backup.
- If DB is down, reads/writes fall back to files.
- Background sync every 60 s: files -> DB (idempotent).

## Auth

mTLS (gateway/tls.rs). Client certificate CN is the operator identity.
