# TODO - rust-iot-concentrator

Приоритеты: P0 - критично, P1 - высокий, P2 - средний, P3 - низкий.

## P0 - блокирует production

- [ ] **P0.5** Группировка устройств по порту (открывать `/dev/ttyM0` один раз, а не N раз за цикл).
  - В `modbus.rs` сгруппировать `cfg.devices` по `port`.
  - Открывать `SerialStream` один раз на порт.
  - Для каждого slave — новый `attach_slave` на клоне или переоткрытии.
- [ ] **P0.7** `poll_ms` из `registers.json`.
  - Добавить `"poll_ms": 50` на верхний уровень `registers.json`.
  - Читать в `ModbusConfig::load`, убрать жёсткие 10 мс.
  - Валидация `quantity <= 125` (Modbus limit).

## P1 - высокая ценность

- [ ] **P1.1** Интеграция с AIG-502 по REST.
  - Чтение `config.json` (ThingsPro twin) через `thingspro-api-v1`.
  - Drift detection (сравнение с baseline).
  - ATT&CK-маппинг событий (30 техник, минимум 10).
- [ ] **P1.3** `cargo test` - проверить, что 4 теста storage проходят (и остальные ~75).
  - `cargo test --lib`
  - `cargo test --test storage_tests`
  - Исправить падающие тесты.
- [ ] **P1.4** Сборка под AIG-502 (x86_64 Linux).
  - `cargo build --release --target x86_64-unknown-linux-gnu`
  - Проверить, что бинарник запускается на Debian 11 (AIG-502).

## P2 - средний приоритет

- [ ] **P2.2** Дашборд Svelte (`ui/`).
  - Планшет с кокпитом: телеметрия, режим, метрики, события.
  - Терминал (xterm.js).
  - mTLS + WSS на 8443.
- [ ] **P2.6** `unsafe` в тесте `balancer` (`degrades_after_hysteresis`).
  - Заменить type punning на `Arc<MockCpu>` + `downcast` через `Any`.
- [ ] **P2.7** eBPF для CPU (`ebpf_cpu.rs`).
  - Сейчас заглушка, работает `/proc/stat`.
  - aya + `.bpf.c` + `PERCPU_ARRAY`.

## P3 - низкий приоритет

- [ ] **P3.8** `README.md` с quick start + `CHANGELOG.md`.
- [ ] **P3.9** Профилирование NUMA (`numastat`, `perf`) и обоснование раскладки.
- [ ] **P3.10** `transport/` - shmem-ring для межплатного обмена (PLATE-A/B/C).

## Сделано

- [x] Модуль 1: структура, `HARDWARE.md`, `Cargo.toml`, `config/`, `certs/`, tests.
- [x] Модуль 2: `core/bus.rs`, `core/mode_switch.rs`.
- [x] Модуль 3: `numa.rs`, `core/dispatcher.rs`, `gateway/` (TLS + mTLS).
- [x] Модуль 4: `hard_pool/`, `monitoring/`, `core/balancer.rs`.
- [x] Модуль 5: `metrics/`, `vm/` (4 домена), `terminal/`.
- [x] Модуль 6: `protocols/` (13 адаптеров, Modbus RTU для AIG-502).
- [x] Модуль 7: `main.rs` (wiring), `systemd/`, `scripts/`, `docs/`.
- [x] Модуль 8: `storage/` (PostgreSQL + JSON fallback), `api/` (REST + SSE), tests, OpenAPI.