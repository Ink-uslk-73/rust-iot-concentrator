# Architecture — Frontier (Telecom IoT Concentrator v4.0)

**Статус:** accepted
**Связанные:** [ADR-0008](../decisions/ADR-0008-frontier-term.md), [ADR-0015](../decisions/ADR-0015-combination-innovation.md), [domain-priority.md](system/domain-priority.md), [glossary.md](glossary.md).

## Layers

```
L0 Hardware: 2x EPYC 7002, 24x HDD, 3 expansion plates (A/B/C)
L1 Transport: PCIe / TSN / shared-memory / NVMe-oF
L2 Core: Bus (4 ch) + Dispatcher + Balancer + ModeSwitch + Watchdog
L3 Domains: HARD (cores 0-7), SOFT (72-95), LIFE (8-31)
L4 Protocols: 13 adapters (modbus/matter/mqtt + DLMS/Router WI/NB-IoT/LTE-M TBD)
L5 Storage: Timescale / Loki / coredump on node 1 (cores 96-127)
L6 UI: Frontier Cockpit (http://127.0.0.1:1967/, REST + SSE, mTLS)
L7 App: src/app/ (config, bus, dispatcher, domains, satellite, storage, api) — main.rs split
```

## Domains (HARD / SOFT / LIFE)

| Уровень | Период | Назначение | Missed tick | NUMA |
|---|---|---|---|---|
| **HARD** | 10 мс | Modbus RTU, тайминги, радиоканал | Skip | node 0, 0-7 |
| **SOFT** | 100 мс | Маршрутизация, состояния, метрики | Skip | node 1, 72-95 |
| **LIFE** | 1000 мс | Логирование, heartbeat, кокпит | Skip | node 0, 8-31 |

Подробнее: [domain-priority.md](system/domain-priority.md).

## Modes (7)

| # | Mode | HARD | SOFT | LIFE |
|---|------|------|------|------|
| 1 | HardOnly | + | - | - |
| 2 | SoftOnly | - | + | - |
| 3 | LifeOnly | - | - | + |
| 4 | HardSoft | + | + | - |
| 5 | HardLife | + | - | + |
| 6 | SoftLife | - | + | + |
| 7 | All | + | + | + |

Degradation: All -> HardSoft -> HardOnly. HARD never degrades.

## FrontierState (7 состояний)

`src/core/frontier.rs` — `FrontierState` (7), `Frontier`, `Scale`, `depth`.

| Состояние | Связь |
|---|---|
| `Wild` | Dew: автономные устройства, сырые данные |
| `Enriched` | парсинг, нормализация |
| `Aggregated` | окна, метрики |
| `Structured` | схема данных |
| `Clustered` | DBSCAN / spectral |
| `Patterned` | motifs / wavelets |
| `Explored` | Cloud: HES/MDMS |

См. [ADR-0008](../decisions/ADR-0008-frontier-term.md).

## main.rs split (app/)

`src/main.rs` — 270 байт (только `fn main()`); `src/app/` — 8 файлов:
`config`, `bus`, `dispatcher`, `domains`, `satellite`, `storage`, `api`, `mod`.

## hard_pool (SLA 10 мс)

`src/hard_pool/` — пул с планировщиком и SLA:
- `worker.rs`: `if dt > 10ms → sla_breaches++` + `DiagLevel::Warning`;
- `scheduler.rs`: `pool.submit(Duration::from_millis(10), ...)`;
- `mod.rs`: `capacity` → `PoolError::Full` + `DiagLevel::Critical`.

## NUMA policy

| Домен | Node | Cores | Scheduler | Политика |
|---|---|---|---|---|
| HARD | 0 | 0-7 | FIFO 90 | node 0, mlockall |
| SOFT | 1 | 72-95 | OTHER | node 1 (strict) |
| LIFE | 0 | 8-31 | OTHER | node 0 (strict) |

`numa_foreign == 0` — health-check. CPU pinning — **P3.25** TBD.

## VM layout

| VM | Node | vCPU | RAM | Политика |
|---|---|---|---|---|
| concentrator-soft | 1 | 72-95 (4 vCPU) | 8 GiB | strict node 1 |
| concentrator-life | 0 | 8-31 (8 vCPU) | 16 GiB | strict node 0 |
| HARD (active) | 0 | 0-7 | (later) | strict node 0 |
| HARD (standby) | 1 | 64-71 | (later) | strict node 1 |

## HARD overflow (Variant C)

- **HARD active** (cores 0-7, node 0): always running.
- **HARD standby** (cores 64-71, node 1): defined but not started (overflow).

## Tablet (Frontier Cockpit)

- Браузер, `http://127.0.0.1:1967/`;
- REST + SSE, mTLS (`docs/api/telemetry.md`);
- См. [ADR-0011](../decisions/ADR-0011-tablet-first-telemetry.md).

## Protocols: downlink/uplink

Downlink: Modbus RTU, OPC UA, PROFINET, EtherNet/IP, CAN, Matter, Zigbee, Thread, BLE.
Uplink: MQTT, HTTP/REST, OPC UA, Modbus TCP.

## Edge computing: Fog node

- **Cloud**: backend (SCADA, cloud, billing) via api/.
- **Fog**: this device (PLATE-A/B/C).
- **Dew** (direct): Matter, Zigbee, Thread, BLE via protocols/soft/.

## Новация (ADR-0015)

Комбинация **DLMS/COSEM** + **NB-IoT** + **LTE-M** + **edge-шлюз (PLATE-B)** + **Frontier (7 состояний)** + **Router WI** — **новация**, зафиксирована в [ADR-0015](../decisions/ADR-0015-combination-innovation.md).

## Ссылки

- [ADR-0008: Фронтир](../decisions/ADR-0008-frontier-term.md) — 7 состояний;
- [ADR-0015: Комбинация](../decisions/ADR-0015-combination-innovation.md) — новация;
- [domain-priority.md](system/domain-priority.md) — HARD/SOFT/LIFE;
- [glossary.md](glossary.md) — термины;
- [telemetry.md](api/telemetry.md) — REST + SSE;
- [TODO.md](../TODO.md) — P2.19 (app/), P2.23 (FrontierState), P3.25 (CPU pinning).
