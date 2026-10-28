# Session notes

## 2026-10-29

### Сделано

- **P2.18b**: Supervisor::spawn_restartable(name, factory) - фабрика + catch_unwind внутри spawn, backoff 1 с, диагностика в Bus. Тест: supervisor::restart_tests::spawn_restartable_restarts_after_panic (1 passed).
- **P2.20c**: реальный heartbeat через alive.<PLATE>. heartbeat_name() -> Option<&'static str> (PLATE-A/B/C). health_check(&Registry, timeout_ms) читает alive.<name>, проверяет now - ts < timeout_ms. modbus/matter/mqtt пишут alive.<PLATE> раз в 1 с в start(). spawn_heartbeat(satellites, name, adapter, metrics, interval, timeout_ms) - 6 аргументов; main.rs вызывает с (..., 2s, 3000).
- **P2.19 восстановлен** после перезаписи main.rs: Dispatcher, cache_tasks, HotCache, RawStore, pg, SatelliteWatchdog, домены Hard/Soft/Life, VmSemaphore, run_domain_with_sem, api::router, CLI.
- **Docs**: README, TODO, docs/glossary.md (UTF-8 без BOM, дубликаты убраны).
- **Push**: 7eb6aac на origin/main, main == origin/main.

### История коммитов

- 7eb6aac docs: dedup README + UTF-8 no BOM
- cfc2cb7 restore P2.19 + P2.20c heartbeat wiring
- cfed006 P2.18b: spawn_restartable; P2.20c: real heartbeat via alive.<PLATE>
- 579abb4 P2.19: Redis -> Raw -> TimescaleDB

### Проверки (зелёные)

- cargo check -> EXIT: 0
- cargo test --lib supervisor::restart_tests -> 1 passed, EXIT: 0

### Дальше

1. **P2.18c** - reconnect-loop mqtt/http + счётчик паник на домен + диагностика в Bus.
2. **P3.19** - AIG-502 failover: Modbus TCP slave на PLATE-C + snapshot из Redis.
3. **P3.20** - проверить Raw CBOR (10 МБ/24 ч) и TimescaleDB hypertable (1s/1m/1h).
4. cargo clippy -- -D warnings + cargo test (полный).
5. Ревью проекта.

Приоритет: P2.18c -> P3.19 -> P3.20 -> ревью.

### Как продолжить в новом чате

1. Вставить в первое сообщение содержимое этого файла.
2. Добавить: Продолжаем с P2.18c. Смотри https://github.com/Ink-uslk-73/rust-iot-concentrator.
3. Приложить git log --oneline -5 и вывод cargo check.

### Правила проекта

- Язык общения: русский. Английский - имена типов/функций, ключевые слова, внешние API.
- Кодировка файлов: UTF-8 без BOM.
- Не использовать heredoc @'...'@ в длинных вставках - рвётся; короткие Add-Content.
