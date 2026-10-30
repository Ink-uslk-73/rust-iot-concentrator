# TODO — rust-iot-concentrator

Приоритеты: **P0** критично, **P1** высокий, **P2** средний, **P3** низкий, **P4** on demand.

**Формат:** `[PN] <путь> — <задача> (status)`.

## P0 — блокирует production

- [x] **P0.5** `src/protocols/hard/modbus_config.rs` — группировка устройств по порту
- [x] **P0.7** `src/protocols/hard/modbus_config.rs` — `poll_ms` из `registers.json` + валидация `quantity <= 125`

## P1 — высокая ценность

- [x] **P1.1** `src/aig/` — AIG-502 REST (client, drift, ATT&CK, poller)
- [x] **P1.3** `cargo test` — все тесты
- [ ] **P1.4** `src/protocols/hard/modbus.rs` — `tokio-modbus` 0.17 (Context, 1 open/порт)
- [x] **P1.5** `migrations/002_timescale.sql` — TimescaleDB hypertable + continuous aggregates + retention
- [ ] **P1.6** `src/transport/{shmem,vsock}.rs` — NUMA-aware transport (DONE: mod/nop/shmem/vsock; **долг:** real SPSC ring в `shmem`, vsock Linux)
- [x] **P1.7** `src/metrics/overhead.rs` — OverheadMetrics (HARD < 1 мс, Bus buffer < 70%/90%)

## P2 — средний приоритет

- [x] **P2.2** `ui/` — Svelte dashboard (Dashboard, ModeSwitch, Telemetry, Events, Terminal, Help)
- [x] **P2.11** `vm/soft` — SOFT в VM (cores 72-95, node 1)
- [x] **P2.12** `vm/life` — LIFE в VM (cores 8-31, node 0)
- [x] **P2.13** `vm/` — единая модель эксплуатации (все домены — VM)
- [x] **P2.14** `src/numa/` — NUMA health-check (`numa_foreign == 0`)
- [x] **P2.15** `vm/hard` — HARD overflow (Вариант C)
- [ ] **P2.16** `emulation/aig-502/` — эмуляция AIG-502 (com0com + OpenModSim): `config.json`, `registers.json`, `README.md`; `scripts/emulation/setup-com0com.ps1`, `check-emulation.ps1`
- [x] **P2.16c** `ui/ConfigMode.svelte` + `src/api/` — UI-переключатель production/emulation
- [ ] **P2.16d** `scripts/emulation/` — установка com0com + OpenModSim (ручной шаг)
- [x] **P2.17** `src/metrics/self_telemetry.rs` — SelfTelemetry (CPU, mem, disks, NUMA, temps)
- [x] **P2.18b** `src/supervisor.rs` — `spawn_restartable` (фабрика + catch_unwind + backoff + Bus diag)
- [x] **P2.18c** `src/transport/reconnect.rs` — `ReconnectSchedule` (equal jitter); `src/supervisor.rs` — `PanicCounter` + Bus diag `n=`
- [ ] **P2.18c-peer** `src/transport/peer_link.rs` — real `LocalP2p::Linux` (сейчас заглушка); `src/storage/store_forward.rs` — миграция на `RawStore`
- [x] **P2.19** `src/app/` (8 файлов: config, bus, dispatcher, domains, satellite, storage, api, mod) — main.rs split (270 байт, только `fn main()`)
- [x] **P2.20c** `src/satellite/heartbeat.rs` — реальный heartbeat `alive.<PLATE>` (health_check + spawn_heartbeat)
- [ ] **P2.21** `src/config/` — `[api] addr` из TOML (fallback `--api-addr`)
- [ ] **P2.22** `src/protocols/dlms.rs` — DLMS/COSEM adapter (HLS-ECDSA, Suite 1/2, X.509) — надстройка, фоном
- [ ] **P2.23** `src/core/frontier.rs` — `FrontierState` (7 состояний), `Frontier`, `Scale`, `depth` (TBD, после docs)
- [ ] **P2.24** `src/protocols/router_telemetry.rs` — SNMP v3 (poll + traps/informs) + NetFlow/IPFIX (collector) + Syslog, Router WI
- [ ] **P2.25** `src/api/telemetry.rs` — SSE `/api/v1/telemetry/stream` (планшет, real-time, KeepAlive 15s)
- [ ] **P2.26** `src/protocols/nb_iot.rs` — NB-IoT adapter (USB-свисток 4G/5G, SIM7080G/SIM8262E-M2), PLATE-B, РФ-диапазоны
- [ ] **P2.27** `src/protocols/lte_m.rs` — LTE-M adapter (Cat-M1), PLATE-B (движущиеся устройства)

## P3 — низкий приоритет

- [ ] **P3.8** `README.md` + `CHANGELOG.md` — актуализировать
- [ ] **P3.9** `src/numa/` — профилирование NUMA (`numastat`, `perf`)
- [ ] **P3.11** `src/api/` — интеграция с внешней SCADA (опционально)
- [ ] **P3.12** `docs/glossary.md` — терминология: Data Concentrator vs Ethernet hub vs Communicator vs Protocol Gateway (DONE частично)
- [x] **P3.13** `src/protocols/hard/modbus_tcp.rs` — Modbus TCP uplink (direct SCADA/HMI)
- [ ] **P3.15** `docs/architecture.md` — Edge computing (Fog node) documentation
- [ ] **P3.18** `ui/` — Tooltip (Pointer Events + Floating UI) + light HelpModal (DONE частично)
- [ ] **P3.19** `src/gateway/` — AIG-502 failover: Modbus TCP slave на PLATE-C + snapshot из Redis
- [ ] **P3.20** `src/storage/raw.rs` — Raw CBOR (10 МБ/24 ч) + TimescaleDB hypertable (1s/1m/1h)
- [ ] **P3.21** `emulation/cooja/` — Cooja (Contiki-NG): WSN-эмуляция (Thread/Zigbee, 6LoWPAN, RPL, 6TiSCH), headless CI
- [ ] **P3.23** `config/satellite/<id>.toml` — конфиги для PLATE-A/B/C (драйверы, СУБД, протоколы), ADR-0005
- [ ] **P3.24** `docs/system/domain-priority.md` — HARD/SOFT/LIFE (10/100/1000 мс), изоляция, backlog, FrontierState
- [ ] **P3.25** `src/vm/` — CPU pinning (taskset/cgroup/affinity), NUMA 0-7/72-95/8-31 — TBD

## P4 — on demand (client request)

- [ ] **P4.1** `src/protocols/` — DNP3 (RS-485/Ethernet) → OPC UA/MQTT
- [ ] **P4.2** `src/protocols/` — IEC 60870-5-104 (substation remote control)
- [ ] **P4.3** `src/protocols/` — BACnet MSTP/IP → Modbus TCP
- [ ] **P4.4** `src/protocols/` — LoRa/wM-Bus/Sigfox → MQTT
- [ ] **P4.5** `src/protocols/` — G3-PLC/PRIME → MQTT/IEC 60870
- [ ] **P4.6** `src/protocols/` — M-Bus (gas/water meters)
- [ ] **P4.7** `src/protocols/` — Proprietary PLC (Siemens S7, Schneider, Mitsubishi, Omron) → OPC UA/MQTT
- [ ] **P4.8** `src/protocols/` — Kafka/AMQP (enterprise integration)
- [ ] **P4.9** `src/protocols/` — Modbus RTU → OPC UA Server
- [ ] **P4.10** `src/protocols/` — User scripts (Lua/Python) for custom preprocessing

## Инфраструктура (один раз при развёртывании)

- [ ] **INF.1** `config/numa.toml` — `lspci -v` → NUMA node для PLATE-A/B/C/HBA
- [ ] **INF.2** `vm/` — `<vcpupin>` + `<numatune mode='strict'>` для каждой VM
- [ ] **INF.3** `GRUB` — `isolcpus=0-7,64-71` (HARD active/standby)
- [ ] **INF.4** `vm/` — `emulatorpin` на housekeeping cores (32-47), **не** на HARD
- [ ] **INF.5** `scripts/emulation/` — настройка эмуляции Modbus RTU (Windows): com0com + OpenModSim

## Сателлиты (R&D)

- [x] **PLATE-A (Hard)** = Moxa AIG-502: полевые протоколы (modbus) реальны; AIG-502 — эмулятор (продакшен, железо не готово)
- [ ] **PLATE-B (Soft)** = умный дом (Matter/Zigbee/Thread/BLE) — **спецификация не готова**
- [ ] **PLATE-C (Life)** = несущая + ASUS + FPGA SmartNIC — скоростные каналы (оптика, WiFi, DAS/JBOD, бэки, облака), часть в разработке

## Долги

- **P1.6** `src/transport/shmem.rs` — SPSC ring (Linux only)
- **P2.18c-peer** `src/transport/peer_link.rs` — `LocalP2p::Linux` real (сейчас заглушка)
- **P2.18c-peer** `src/storage/store_forward.rs` — миграция на `RawStore`
- **ADR-0005** `docs/decisions/ADR-0005-satellite-driver-matrix.md` — таблица драйверов/СУБД/протоколов PLATE-A/B/C
- **ADR-0007** `docs/decisions/ADR-0007-dlms-cosem-adapter.md` — создан (DLMS adapter)
- **ADR-0008** `docs/decisions/ADR-0008-frontier-term.md` — создан (Фронтир, 7 состояний)
- **ADR-0009** `docs/decisions/ADR-0009-project-name-frontier.md` — создать (имя проекта `Frontier`, bin, UI, API)
- **P3.19** `src/gateway/` — AIG-502 failover (Modbus TCP slave на PLATE-C + snapshot из Redis)
- **P3.20** `src/storage/raw.rs` — Raw CBOR (10 МБ/24 ч) + TimescaleDB hypertable (1s/1m/1h)
- **PLATE-B (Soft)** — спецификация не готова
- **docs** — `ADR-0005-satellite-driver-matrix.md` (таблица драйверов/СУБД/протоколов)
- **docs** — `config/satellite/<id>.toml` (конфиги для A/B/C)
- **Инфраструктура** — `INF.1..INF.5`

## Отдельные репозитории

- [x] **rust-iot-concentrator-tablet** — Flutter-клиент (создан, README + TODO)

## Сделано

- Модули 1–8 (структура, core, numa, dispatcher, gateway, hard_pool, monitoring, balancer, metrics, vm, terminal, protocols, main, systemd, scripts, docs, storage, api, tests)
- Архитектурные решения: микро-SCADA/HMI в LIFE, TimescaleDB (не Cassandra), NUMA-раскладка (HARD active node 0, standby node 1, SOFT node 1, LIFE node 0, STORAGE node 1), HARD overflow (Вариант C)
- P0.5, P0.7, P1.1, P1.3, P1.5, P1.7, P2.2, P2.11, P2.12, P2.13, P2.14, P2.15, P2.16c, P2.17, P2.18b, P2.18c (reconnect/PanicCounter), P2.19, P2.20c, P3.13

> **Статус: R&D.** Контракты фиксируются раньше реализации. Эмуляторы — штатно.

