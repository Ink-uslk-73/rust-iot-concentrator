# Telemetry API (REST + SSE, mTLS)

**Статус:** контракт (ADR-0011)
**Связанные:** [ADR-0011](../decisions/ADR-0011-tablet-first-telemetry.md), [ADR-0010](../decisions/ADR-0010-router-telemetry.md), [router-telemetry.md](../protocols/router-telemetry.md), [glossary](../glossary.md).

## REST (планшет → frontier)

| Метод | Путь | Назначение |
|---|---|---|
| GET | `/api/v1/health` | `frontier: ok`, version, name |
| GET | `/api/v1/telemetry?from=...&to=...&metric=...` | история (TimescaleDB) |
| GET | `/api/v1/events?limit=...` | события (Bus diagnostics) |
| GET | `/api/v1/satellites` | статус PLATE-A/B/C (alive) |
| GET | `/api/v1/metrics` | overhead metrics |
| GET | `/api/v1/metrics/self` | self-telemetry |

## SSE (real-time)

- **`GET /api/v1/telemetry/stream`** — поток телеметрии (Bus `Telemetry`);
- **`KeepAlive 15s`** — чтобы proxy не закрыл соединение;
- **`Last-Event-ID`** — resume (если соединение упало);
- **`id`** — seq (Bus `next_seq`);
- **`event: telemetry`**, `data: {...}` (JSON).

**Формат:**

```
event: telemetry
id: 42
data: {"sensor_id":"s1","value":42.0,"unit":"C","timestamp_ms":1730000000000}

event: telemetry
id: 43
data: {...}
```

## mTLS

- **Клиент** **планшета** — **сертификат** (`certs/`), **CN** = `tablet-*`;
- **Сервер** — `certs/`, mTLS (`docs/api/`);
- **CN** **проверяется** (**allowlist** `tablet-*`).

## Почему SSE, а не WebSocket

См. [ADR-0011](../decisions/ADR-0011-tablet-first-telemetry.md) — SSE (server → client, auto-reconnect, HTTP) **проще** WebSocket.

## Ссылки

- [ADR-0011: tablet-first](../decisions/ADR-0011-tablet-first-telemetry.md) — REST + SSE, mTLS;
- [ADR-0010: Router WI](../decisions/ADR-0010-router-telemetry.md) — источник (SNMP/NetFlow);
- [router-telemetry.md](../protocols/router-telemetry.md) — SNMP/NetFlow/Syslog;
- [glossary](../glossary.md) — термины `SSE`, `mTLS`;
- [TODO.md](../../TODO.md) — P2.25 (SSE).
