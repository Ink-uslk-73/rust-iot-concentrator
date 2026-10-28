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
