# ADR-0010: Router WI telemetry (SNMP v3 + NetFlow/IPFIX + Syslog)

**Статус:** принято
**Дата:** 2026-10-30

## Контекст

Router WI — **реальный** источник телеметрии (не эмуляция). Требуется сбор:
- **SNMP v2c/v3** (poll + traps/informs);
- **NetFlow v9 / IPFIX** (collector);
- **Syslog** (collector);
- **sFlow**, **gNMI** (опционально).

## Решение

- **`ProtocolAdapter`** в `src/protocols/router_telemetry.rs` (**P2.24**);
- **SNMP v3** (auth/priv) — **production**; v2c — **fallback** для старых;
- **NetFlow/IPFIX collector** на `frontier` (UDP 2055);
- **Syslog collector** на `frontier` (UDP 514);
- **SNMP traps/informs** — UDP 162 (inform — для критичных);
- **sFlow** — UDP 6343; **gNMI** — TCP 57400 (TLS);
- **Данные** → **Bus** (`Telemetry`) → **TimescaleDB**;
- **`FrontierState`** — `Wild` → `Enriched` (SNMP poll) → `Aggregated` (NetFlow).

## Обоснование

- **Router WI** — **реальный** источник (**не** эмуляция);
- **SNMP v3** — безопасность; **Trap/Inform** — события;
- **NetFlow/IPFIX** — **кто/куда/сколько** трафика;
- **`FrontierState`** — **связь** с **ADR-0008** (7 состояний).

## Что НЕ входит

- **Реализация** `src/protocols/router_telemetry.rs` — **P2.24**, **TBD**;
- **NetFlow analytics** (DBSCAN/spectral) — **P3**, **после** TimescaleDB.

## Последствия

- `docs/protocols/router-telemetry.md` — **инженерное** **описание**;
- `TODO.md` — `P2.24` (router_telemetry.rs), `P2.25` (SSE);
- `src/protocols/router_telemetry.rs` — **создать** (TBD, P2.24).

## Ссылки

- [router-telemetry.md](../protocols/router-telemetry.md) — SNMP/NetFlow/sFlow/gNMI/Syslog;
- [ADR-0008: Фронтир](ADR-0008-frontier-term.md) — 7 состояний;
- [telemetry.md](../api/telemetry.md) — контракт для планшета;
- [glossary.md](../glossary.md) — термины;
- [TODO.md](../../TODO.md) — P2.24 (Router WI).
