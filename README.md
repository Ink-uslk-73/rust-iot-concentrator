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