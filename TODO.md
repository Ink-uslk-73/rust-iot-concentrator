# TODO - rust-iot-concentrator

Приоритеты: P0 - критично, P1 - высокий, P2 - средний, P3 - низкий.

## P0 - блокирует production

- [x] **P0.5** Группировка устройств по порту.
- [x] **P0.7** `poll_ms` из `registers.json` + валидация `quantity <= 125`.

## P1 - высокая ценность

- [x] **P1.1** Интеграция с AIG-502 по REST (client, drift, ATT&CK, poller).
- [x] **P1.3** `cargo test` - все тесты проходят.
- [ ] **P1.4** Обновление `tokio-modbus` до 0.17 (переиспользование `Context`).

## P2 - средний приоритет

- [ ] **P2.2** Дашборд Svelte (`ui/`).
- [ ] **P2.11** Вынести SOFT в отдельную VM (cores 8-31, node 0).
- [ ] **P2.12** Вынести LIFE в отдельную VM (cores 72-95, node 1).
- [ ] **P2.13** Единая модель эксплуатации: все три домена - VM.

## P3 - низкий приоритет

- [ ] **P3.8** `README.md` с quick start + `CHANGELOG.md`.
- [ ] **P3.9** Профилирование NUMA.
- [ ] **P3.10** `transport/` - shmem-ring для межплатного обмена.

## Отдельные репозитории

- [x] **`rust-iot-concentrator-tablet`** - Flutter-клиент (Android 8.0+, Windows).
  - Репозиторий: https://github.com/Ink-uslk-73/rust-iot-concentrator-tablet
  - Статус: создан, README + TODO.

## Сделано

- [x] Модуль 1-8 (структура, core, numa, dispatcher, gateway, hard_pool, monitoring, balancer, metrics, vm, terminal, protocols, main, systemd, scripts, docs, storage, api, tests).
