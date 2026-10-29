<<<<<<< HEAD
﻿# rust-iot-concentrator v4.0

Телекоммуникационный концентратор: HARD (10 мс) / SOFT (100 мс) / LIFE (1000 мс),
2x AMD EPYC 7002, 24x HDD, планшет с кокпитом по mTLS.

## Модуль 1 (готов)
- HARDWARE.md, Cargo.toml, config/*.toml, certs/README.md, tests/*
=======
# rust-iot-concentrator
Телекоммуникационный концентратор v4.0
>>>>>>> bd72580983e608ccd2e6770e32f641efa13ee9a8
---

## Терминология

**Концентратор данных (Data Concentrator)** - агрегатор телеметрии с N полевых
устройств (Modbus RTU, OPC UA, Matter, Zigbee) в один uplink (REST/SSE/UI).

**Не путать** с:
- **Ethernet hub** (устаревшее устройство 1-го уровня OSI, рассылает на все порты).
- **Protocol Gateway** (трансляция протоколов: Modbus -> MQTT).
- **Communicator** (связность: сотовый роутер, VPN).

См. docs/glossary.md.
## Protocols

**Downlink** (field -> concentrator): Modbus RTU, OPC UA, PROFINET, EtherNet/IP, CAN,
Matter, Zigbee, Thread, BLE. (DNP3, BACnet, LoRa, M-Bus, G3-PLC, S7 - on demand.)

**Uplink** (concentrator -> backend): MQTT, HTTP/REST, OPC UA, Modbus TCP.

**Above conversion**: normalization, aggregation, filtering, buffering (store & forward),
user scripts (Lua/Python) - see docs/glossary.md.
## Edge computing (Fog node)

This concentrator is a **Fog node** (edge-DC), not just a protocol converter:

- **Cloud**: backend (SCADA, cloud, billing) - via api/ (REST/SSE).
- **Fog**: this device (PLATE-A + PLATE-B -> PLATE-C).
- **Dew**: direct Matter/Zigbee/Thread/BLE (protocols/soft/).

**5G MEC**: NFV principles (decomposition, orchestration). See docs/glossary.md.
---

## Язык общения

Весь проект (README, TODO, glossary, комментарии в коде, commit messages)
ведётся на **русском языке**. Английский  только для имён типов/функций,
ключевых слов и внешних API (например, MQTT, OPC UA, Modbus).

## Satellite-платы (архитектура)

- **PLATE-A** (HARD, 10 мс)  Modbus RTU / Modbus TCP / PROFINET / EtherNet/IP / CAN / OPC UA.
- **PLATE-B** (SOFT, 100 мс)  Matter / Zigbee / Thread / BLE.
- **PLATE-C** (LIFE, 1000 мс)  MQTT / HTTP / REST / OPC UA uplink / Modbus TCP uplink.

Каждая плата  отдельный процесс/адаптер. Живость каждой подтверждается
метрикой `alive.<PLATE>` в `Registry` (адаптер пишет `now_ms` раз в секунду
из своего цикла). `spawn_heartbeat` проверяет свежесть и дергает
`SatelliteRegistry::heartbeat(name)`. Если `alive` старше `timeout_ms` 
сателлит считается мёртвым, heartbeat не идёт.

## Тройное резервирование (Triple redundancy)

1. **Три сателлитные платы A/B/C**  независимые процессы, независимые
   адаптеры, независимые heartbeat. Падение одной не останавливает две другие.
2. **AIG-502**  резервный шлюз (аппаратный). Включается, если CORE-узел
   недоступен: держит последний снимок телеметрии и отдаёт по Modbus TCP.
3. **CORE  три уровня данных**:
   - **Redis (hot-path)**  последние значения, TTL 60 с, O(1) чтение для UI/API.
   - **Raw CBOR**  бинарный лог без потерь, сегменты по 10 МБ, ротация 24 ч.
   - **TimescaleDB**  агрегаты (1 с / 1 мин / 1 ч), долговременное хранение.

Итог: при отказе CORE  работает AIG-502; при отказе AIG-502  остаются
сателлиты A/B/C с локальным буфером store & forward (Raw CBOR).
Отказ любой одной платы не роняет концентратор.

## P2.18b  spawn_restartable

`Supervisor::spawn_restartable(name, factory)`  фабрика вызывается заново
при каждой панике (catch_unwind внутри spawn, backoff 1 с). Не применять для
задач с уникальными ресурсами (`mpsc::Receiver`, `TcpStream` и т.п.) 
для них обычный `spawn` + внутренний reconnect-loop.

## P2.20c  реальный heartbeat

`ProtocolAdapter::health_check(&Registry, timeout_ms)`  читает
`alive.<name>` и сравнивает `now - ts < timeout_ms`. Адаптер пишет
`alive` в своём цикле `start()`. `spawn_heartbeat` вызывает
`health_check` и, только если `true`,  `registry.heartbeat(name)`.

## Статус проекта

**R&D (research & development).** Архитектура (HARD/SOFT/LIFE, triple redundancy, CORE: Redis/Raw/TimescaleDB) зафиксирована. Реализация  поэтапная: контракты и эмуляторы -> реальное железо. AIG-502 — в продакшене, пока используется эмулятор (emulation/aig-502/). PLATE-A (HARD) — полевые протоколы + AIG-502 (эмулятор, пока железо не готово). PLATE-B (SOFT) — умный дом (Matter/Zigbee/Thread/BLE), эмуляторы, в разработке. PLATE-C (LIFE) — скоростные каналы (оптика, WiFi, DAS/JBOD, бэки, облака), часть каналов в разработке. PLATE-A (HARD)  полевые протоколы + AIG-502 (эмулятор, пока железо не готово). PLATE-B (SOFT)  умный дом (Matter/Zigbee/Thread/BLE), эмуляторы, в разработке. PLATE-C (LIFE)  скоростные каналы (оптика, WiFi, DAS/JBOD, бэки, облака), часть каналов в разработке.

## Системотехника (интерконнект и питание)

- **QPI/UPI/Infinity Fabric**  межпроцессорная шина (2 сокета, когерентная память). **Не** для периферии.
- **PCIe**  периферия и **сателлиты**. Сателлиты интегрированы в PCIe, обмениваются сигналами и данными **между собой** (P2P DMA) и с CORE.
- **Питание сателлита  трёхуровневое**: (1) свой БП (основной) -> (2) PCIe (резерв, если свой БП отказал) -> (3) ИБП (последний рубеж, корректный shutdown).
- **Требования к железу** (фиксируются до проектирования): root complex поддерживает PCIe P2P DMA (ACS настроен, IOMMU настроен); BIOS разрешает PCIe P2P; питание PCIe достаточно для резерва сателлита.
- **Порядок**: сначала архитектура и ПО с эмуляторами (фиксация контрактов), потом проектирование железа. Управление рисками, не компромисс.

### Сателлит  отдельный компьютер

Каждый сателлит (PLATE-A/B/C)  **отдельный компьютер** в стойке (концентратор  4U). Свой энергоисточник + ИБП. Режимы: автономная перезагрузка (не останавливает концентратор); смена версии ПО (не останавливает остальные сателлиты); автономная диагностика/тесты.

**Store & forward**: если концентратор перегружен или сбоит, сателлит пишет Raw **локально** (CBOR, 10 МБ сегменты, ротация 24 ч). Когда концентратор оживает  сателлит досылает Raw в CORE по PCIe P2P (или концентратор забирает по запросу).

### Четыре уровня данных

```nSatellite-Raw (локально, store & forward, CBOR 10 МБ/24 ч)
   | PCIe P2P (досылка при оживлении CORE)
CORE-Raw (CBOR, 10 МБ/24 ч)
   | flush (60 с)
Redis (hot-path, TTL 60 с, O(1))
   | sync (агрегаты)
TimescaleDB (1s / 1m / 1h)
```

**Резервирование**: отказ HARD не роняет SOFT/LIFE; отказ CORE не роняет сателлиты (пишут Raw локально); отказ PCIe P2P не роняет сателлиты (store & forward, досылка позже). **Кворум не требуется**  роли плат **не пересекаются**.

### Heartbeat

- `alive.<PLATE>`  **компьютер жив**, пишется **агентом на сателлите** раз в 1 с.
- `health_check(&Registry, timeout_ms)`  `now - alive_ts < timeout_ms` (HARD  3000 мс, SOFT/LIFE  настраивается).
- `spawn_heartbeat(satellites, name, adapter, metrics, interval, timeout_ms)`  читает `health_check`, дергает `SatelliteRegistry::heartbeat(name)`.
- Перезагрузка сателлита: агент стартует заново -> `alive` обновляется. Сетевой/PCIe разрыв: `alive` стареет -> `health_check = false` -> концентратор знает, что сателлит недоступен, но сателлит **продолжает писать Raw локально**.

## Оценки эффективности и производительности

**Это оценки порядка величины, не бенчмарки.** Реальные цифры  после эмуляторов и HIL. Допущения указаны явно.

### Latency (HARD, 10 мс)

| Компонент | Оценка | Допущение |
|---|---|---|
| Modbus RTU опрос (1 регистр) | 15 мс | 9600115200 бод, 18 байт |
| Bus `try_send` (HARD) | < 1 мкс | mpsc, capacity 256 |
| Dispatcher -> HardDomain | 1050 мкс | tokio task, no contention |
| `alive.<PLATE>` запись | < 10 мкс | in-memory Registry |
| **Итого HARD (без PCIe)** | **~26 мс** | в пределах 10 мс |

PCIe P2P (если AIG-502 отдаёт данные через PCIe): +1050 мкс на DMA, пренебрежимо на фоне Modbus.

**Вывод**: HARD укладывается в 10 мс с запасом ~2x. Запас нужен на jitter (ОС, планировщик, IRQ). **Граница**: при Modbus RTU 9600 бод опрос 8 регистров может занять > 10 мс  нужен буфер или более высокая скорость.

### Throughput

| Канал | Оценка | Реально (5070%) |
|---|---|---|
| PCIe 4.0 x16 | ~32 ГБ/с | 1622 ГБ/с |
| PCIe 5.0 x16 | ~64 ГБ/с | 3245 ГБ/с |
| 10 GbE | ~1.25 ГБ/с | 0.91.1 ГБ/с |
| 25 GbE | ~3.1 ГБ/с | 2.32.8 ГБ/с |
| 100 GbE | ~12.5 ГБ/с | 911 ГБ/с |

**PCIe P2P быстрее сети в 1530x** (PCIe 4.0 vs 10 GbE) и **в 510x** (PCIe 4.0 vs 100 GbE). Для LIFE (DAS/JBOD, бэки, облака) критично: 1 ТБ по PCIe 4.0  ~1 мин, по 10 GbE  ~15 мин, по 100 GbE  ~1.5 мин.

### Восстановление

| Сценарий | Время | Что происходит |
|---|---|---|
| Отказ своего БП сателлита | < 1 с | переключение на PCIe-питание (аппаратно) |
| Отказ PCIe-питания | < 10 с | переключение на ИБП -> корректный shutdown |
| Перезагрузка сателлита | 30120 с | агент стартует, `alive` обновляется, Raw досылается |
| Смена ПО сателлита | 15 мин | остальные работают, Raw копится локально |
| Отказ CORE | 0 (сателлиты живут) | Raw пишется локально, досылка при оживлении |
| Отказ PCIe P2P | 0 (сателлиты живут) | Raw пишется локально, досылка при восстановлении |
| Отказ HARD-адаптера (A) | 0 (B/C живут) | `alive.PLATE-A` стареет -> `health_check = false` |
| Отказ AIG-502 | 0 (HARD деградирует) | Modbus TCP fallback на PLATE-A; snapshot из Redis |

**Вывод**: все отказы  **локальные**, не каскадные. Это и есть triple redundancy: роли не пересекаются.

### Энергия (оценка)

| Параметр | Оценка |
|---|---|
| Концентратор (4U) | 8001500 Вт |
| Сателлит | 300600 Вт |
| 3 сателлита + концентратор | 1.73.3 кВт |
| Тепло | 5.811.3 kBTU/ч |
| ИБП | 35 кВА на стойку (запас 2030%) |

Требует расчёта тепла и энергобаланса  системотехника, не ПО.

## Границы применимости

1. Оценки latency/throughput  порядок величины, не бенчмарки.
2. PCIe P2P требует поддержки root complex (ACS, IOMMU, BIOS). Если плата не умеет  P2P не работает, throughput падает до сети (10/25/100 GbE).
3. Трёхуровневое питание  аппаратное. Если схема не переключается автоматически  оценки < 1 с неверны.
4. HARD 10 мс  при Modbus RTU 9600 бод опрос 8 регистров может занять > 10 мс  нужен буфер или более высокая скорость.
5. 4U / 1.73.3 кВт  оценка для 2x EPYC 7002 + 24 HDD + 3 сателлита. Реальная цифра  после выбора железа.