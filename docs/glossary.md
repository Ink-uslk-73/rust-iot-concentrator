# Glossary - rust-iot-concentrator

| Term | Meaning | Do not confuse with |
|------|---------|---------------------|
| **Data Concentrator** (Концентратор данных) | Device/software that aggregates data from N field devices (Modbus, OPC UA, Matter, Zigbee) into 1 uplink (REST/SSE/UI). | Ethernet hub (L1, obsolete, floods all ports). |
| **Protocol Gateway** (Протокольный шлюз) | Device/software that translates one protocol to another (Modbus -> MQTT, Modbus -> OPC UA). | Data Concentrator (aggregates N -> 1). |
| **Communicator** (Коммуникатор) | Device that provides connectivity between two points (cellular router, VPN gateway). | Data Concentrator (aggregation), Protocol Gateway (translation). |
| **Ethernet hub** | Obsolete L1 OSI device, floods signal to all ports. Not used in IIoT. | Data Concentrator (our project). |

## Why it matters

- In Russian, "концентратор" can mean **Ethernet hub** (obsolete) or **Data Concentrator** (IIoT).
- Our project is **Data Concentrator**: PLATE-A (Modbus) + PLATE-B (Matter/Zigbee) -> PLATE-C (uplink) -> API/UI.
- **Protocol Gateway** is a separate role (e.g. Modbus -> MQTT). Not the same as concentrator.
- **Communicator** is PLATE-C (Router/Gateway: VPN, NAT, bridges). Not the concentrator core.

## Data Concentrator: protocols

### Downlink (field devices -> concentrator)

| Protocol | Channel | Typical use | In project |
|----------|---------|-------------|------------|
| Modbus RTU/ASCII | RS-485 | Sensors, meters, PLC (most common) | YES protocols/hard/modbus.rs |
| Modbus TCP | Ethernet | Industrial controllers | P3.13 |
| DNP3 | RS-485/Ethernet | Energy, SCADA | P4.1 (on demand) |
| IEC 60870-5-104 | Ethernet | Substation remote control | P4.2 (on demand) |
| BACnet MSTP/IP | RS-485/Ethernet | Building automation | P4.3 (on demand) |
| PROFIBUS DP/PROFINET | RS-485/Ethernet | Industrial automation | YES protocols/hard/profinet.rs (stub) |
| DeviceNet/CANopen | CAN | Simple I/O, drives | YES protocols/hard/can.rs (stub) |
| LoRa/wM-Bus/Sigfox | LPWAN | Smart meters, environmental | P4.4 (on demand) |
| G3-PLC/PRIME | PLC | AMI | P4.5 (on demand) |
| M-Bus | RS-485 | Gas/water meters | P4.6 (on demand) |
| Proprietary PLC (S7, Schneider, ...) | RS-485/Ethernet | Bypass "foreign" PLC | P4.7 (on demand) |

### Uplink (concentrator -> backend)

| Protocol | Purpose | In project |
|----------|---------|------------|
| MQTT (+ JSON) | Cloud/IoT platform (ThingsBoard, AWS IoT, Azure IoT) | YES protocols/life/mqtt.rs (stub) |
| OPC UA (Client/Server, Pub/Sub) | MES, SCADA, digital twin | YES protocols/hard/opcua.rs (stub) |
| Modbus TCP | Direct SCADA/HMI | P3.13 |
| HTTP/REST / AMQP / Kafka | Enterprise integration | YES api/ (REST) + P4.8 (Kafka) |

### Typical conversion scenarios

| Scenario | In project |
|----------|------------|
| Modbus RTU -> Modbus TCP | P3.13 |
| Modbus RTU -> MQTT + JSON | P2.16d (emulation) + P1.5 (TimescaleDB) |
| Modbus RTU -> OPC UA Server | P4.9 |
| DNP3 -> OPC UA/MQTT | P4.1 |
| BACnet MSTP -> Modbus TCP | P4.3 |
| LoRa/wM-Bus -> MQTT | P4.4 |
| G3-PLC -> MQTT/IEC 60870 | P4.5 |
| Siemens S7 -> OPC UA/MQTT | P4.7 |

### What the concentrator does "above" pure conversion

1. **Normalization**: engineering units (deg C, kWh, bar).
2. **Aggregation**: N points -> 1 packet (Bus + Dispatcher).
3. **Filtering/triggers**: send only on threshold (Dispatcher + aig/attck.rs).
4. **Buffering**: local store on link loss (store & forward) (storage/ + JSON fallback).
5. **User scripts** (Lua, Python): custom preprocessing (bot/fsm.rs, P4.10).

This is why modern concentrators (Robustel, Advantech, BLIoT) are called
**edge devices**, not just "protocol converters".

## Cloud-Fog-Dew (edge computing model)

| Layer | What | In project |
|-------|------|------------|
| **Cloud** | Centralized DC (backend: SCADA, cloud, billing) | api/ (REST/SSE) -> uplink |
| **Fog** | Local cloud (edge-DC, enterprise DC) | **This concentrator** (PLATE-A/B -> PLATE-C) |
| **Dew** | Direct end-to-end (Wi-Fi, BLE, ZigBee) | protocols/soft/ (Matter, Zigbee, Thread, BLE) |

**Why it matters:**
- Edge computing: functions move closer to usage (latency, bandwidth).
- 5G MEC: edge in mobile networks (NFVI + middleware).
- Concentrator = **Fog node** + **Dew connectivity** + **Cloud uplink**.
- Traffic growth (big data, video, tactile internet, autonomous) -> transport boom.