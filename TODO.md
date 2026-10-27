# TODO - rust-iot-concentrator

Приоритеты: P0 - критично, P1 - высокий, P2 - средний, P3 - низкий.

## P0 - блокирует production

- [x] **P0.5** Группировка устройств по порту.
- [x] **P0.7** `poll_ms` из `registers.json` + валидация `quantity <= 125`.

## P1 - высокая ценность

- [x] **P1.1** AIG-502 REST (client, drift, ATT&CK, poller).
- [x] **P1.3** `cargo test` - все тесты.
- [ ] **P1.4** `tokio-modbus` 0.17 (переиспользование `Context`, 1 open на порт).
- [ ] **P1.5** TimescaleDB (через shell, после UI).
  - `scripts/storage/install-timescaledb.sh`, `check-timescaledb.sh`, `migrate.sh`.
  - `migrations/002_timescale.sql`: hypertable + continuous aggregates + retention + compression.
  - **Где:** STORAGE VM (node 1, cores 96-127).
  - **Осторожно:** `start_offset` continuous aggregate > `drop_after` retention.
- [ ] **P1.6** NUMA-aware `transport/`.
  - shmem-ring на node producer (HARD -> node 0).
  - vsock для cross-node (CORE <-> SOFT/LIFE/STORAGE), батчинг.
  - TCP loopback - не использовать для HARD.
  - **Зависит от:** P2.11-P2.12 (вынос в VM).
  - [ ] **P1.7** Overhead metrics (added latency, throughput, jitter, buffer).
  - `src/metrics/overhead.rs`: `OverheadMetrics` (atomic counters).
  - HARD: < 1 мс (`modbus.rs` → `bus.send`).
  - API: < 1 мс (`api/` handler).
  - SSE: jitter < 10 мс.
  - Bus buffer: < 70% (warning), > 90% (critical).
  - `GET /api/v1/metrics/overhead` (JSON).
  - Alert в diagnostics при превышении порогов.
  - **Зависит от:** P2.2 (UI) — чтобы отображать метрики.
  - **До** P2.11–P2.12 (VM) — чтобы знать overhead до выноса.

## P2 - средний приоритет

- [x] **P2.2** Дашборд Svelte (`ui/`): Dashboard, ModeSwitch, Telemetry, Events, Terminal, Help (modal, tooltips, autocomplete, thresholds). Встроен в бинарник (`rust-embed`).
- [ ] **P2.11** Вынести SOFT в VM (cores 72-95, node 1).
  - `<vcpupin>` + `<numatune mode='strict' nodeset='1'>`.
  - `MPOL_BIND`, shmem на node 1.
  - **Зависит от:** `lspci -v` (подтвердить PLATE-B на node 1).
- [ ] **P2.12** Вынести LIFE в VM (cores 8-31, node 0).
  - `<vcpupin>` + `<numatune mode='strict' nodeset='0'>`.
  - `MPOL_BIND`, shmem на node 0.
  - **Зависит от:** `lspci -v` (подтвердить PLATE-C на node 0).
- [ ] **P2.13** Единая модель эксплуатации: все домены - VM.
  - **Ограничение:** делегирование ядер **только внутри ноды** (cross-node запрещён для HARD).
  - `virsh setvcpus --live` (внутри ноды), cgroup v2.
- [ ] **P2.14** NUMA health-check.
  - `numastat -c qemu-kvm` -> `numa_foreign` должен быть 0.
  - Alert (`DiagLevel::Critical`) при `numa_foreign > 0`.
  - На housekeeping ядрах (32-47), **не** на HARD (0-7).
  - `lspci -v` - **один раз** при развёртывании, результат в `config/numa.toml`.
- [ ] **P2.15** HARD overflow (Вариант C).
  - HARD standby (node 1) **не запущен** в норме.
  - При перегрузке node 0 - включить standby, разделить устройства (`/dev/ttyM1`).
  - При спаде - выключить standby.
  - **Не миграция** - разделение устройств.

## P3 - низкий приоритет

- [ ] **P3.8** `README.md` + `CHANGELOG.md`.
- [ ] **P3.9** Профилирование NUMA (`numastat`, `perf`).
- [ ] **P3.11** Интеграция с внешней SCADA (опционально).
  - Концентратор отдаёт данные через API; SCADA читает.
  - Не встраивать тяжёлую SCADA в LIFE.

## Отдельные репозитории

- [x] **`rust-iot-concentrator-tablet`** - Flutter-клиент.
  - Репозиторий: https://github.com/Ink-uslk-73/rust-iot-concentrator-tablet
  - Статус: создан, README + TODO.

## Инфраструктура (один раз при развёртывании)

- [ ] **INF.1** `lspci -v` на сервере -> `NUMA node` для PLATE-A/B/C/HBA -> `config/numa.toml`.
- [ ] **INF.2** `<vcpupin>` + `<numatune mode='strict'>` для каждой VM.
- [ ] **INF.3** `isolcpus=0-7,64-71` в GRUB (HARD active/standby).
- [ ] **INF.4** `emulatorpin` на housekeeping cores (32-47), **не** на HARD.

## Сделано

- [x] Модуль 1-8 (структура, core, numa, dispatcher, gateway, hard_pool, monitoring, balancer, metrics, vm, terminal, protocols, main, systemd, scripts, docs, storage, api, tests).
- [x] P0.5, P0.7, P1.1, P1.3.
- [x] Архитектурные решения: микро-SCADA/HMI в LIFE, не монолит; TimescaleDB (не Cassandra); NUMA-раскладка (HARD active node 0, standby node 1, SOFT node 1, LIFE node 0, STORAGE node 1); HARD overflow (Вариант C).

- [ ] **P2.16** Эмуляция AIG-502 для отладки (без реального железа).
  - **Слой 1 (Modbus RTU):** com0com (пара COM10 <-> COM11) + OpenModSim (Slave на COM11, slave ID = 1, holding registers 0-10).
  - **Слой 2 (конфигурация):** локальные config.json (device: "COM10") + egisters.json в emulation/aig-502/.
  - **Слой 3 (HTTP API AIG, опционально):** mock HTTP-сервер для ig/client.rs (drift, ATT&CK).
  - **Структура:** emulation/aig-502/{config.json, registers.json, README.md}.
  - **Инструменты:** scripts/emulation/setup-com0com.ps1, scripts/emulation/check-emulation.ps1.
  - **Зависит от:** P0.7 (config.json/registers.json парсинг уже есть).

- [ ] **INF.5** Настройка эмуляции Modbus RTU (Windows).
  - Установить com0com (админ, тест-подпись драйвера).
  - Создать пару COM10 <-> COM11 (через setupg.exe или CLI).
  - Запустить OpenModSim: RTU Slave на COM11, slave ID = 1, holding registers 0-10.
  - Проверить: концентратор открывает COM10, читает регистры из OpenModSim.