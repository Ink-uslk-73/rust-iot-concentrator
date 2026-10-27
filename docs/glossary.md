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
