# Architecture - Telecom Concentrator v4.0

## Layers

```
L0 Hardware: 2x EPYC 7002, 24x HDD, 3 expansion plates (A/B/C)
L1 Transport: PCIe / TSN / shared-memory / NVMe-oF
L2 Core: Bus (4 ch) + Dispatcher + Balancer + ModeSwitch + Watchdog
L3 Domains: HARD (cores 0-7, FIFO 90), SOFT (8-31), LIFE (64-95)
L4 Protocols: 5 hard + 4 soft + 4 life = 13 adapters
L5 Storage: Timescale / Loki / coredump on node 1 (cores 96-127)
L6 Gateway: TLS + mTLS + WSS (8443 dashboard, 8444 terminal)
```

## Modes (7)

| # | Mode | HARD | SOFT | LIFE |
|---|------|------|------|------|
| 1 | HardOnly | + | - | - |
| 2 | SoftOnly | - | + | - |
| 3 | LifeOnly | - | - | + |
| 4 | HardSoft | + | + | - |
| 5 | HardLife | + | - | + |
| 6 | SoftLife | - | + | + |
| 7 | All | + | + | + |

Degradation: All -> HardSoft -> HardOnly. HARD never degrades.

## NUMA policy

| Domain | Node | Cores | Scheduler | Memory |
|--------|------|-------|-----------|--------|
| HARD | 0 | 0-7 | FIFO 90 | node 0, mlockall |
| SOFT | 0 | 8-31 | OTHER | node 0 |
| CORE | 0 | 32-47 | OTHER | node 0 |
| LIFE | 1 | 64-95 | OTHER | node 1 |
| STORAGE | 1 | 96-127 | OTHER | node 1 |

## Tablet

- Wi-Fi VLAN 30, WPA3-Enterprise
- mTLS client certificate (certs/tablet-NN.crt)
- Dashboard: wss://concentrator:8443
- Terminal: wss://concentrator:8444 (allowlist + audit)

## Tests

```
cargo test
```

## Terminology

**Data Concentrator** (this project): aggregates N field devices -> 1 uplink.
**Not** Ethernet hub (L1, obsolete). **Not** Protocol Gateway (translation).
**Not** Communicator (connectivity). See docs/glossary.md.

## Protocols: downlink/uplink

**Downlink** (field -> concentrator): Modbus RTU, Modbus TCP, DNP3, IEC 60870-5-104,
BACnet, PROFINET, CAN, LoRa, G3-PLC, M-Bus, proprietary PLC.

**Uplink** (concentrator -> backend): MQTT, OPC UA, Modbus TCP, HTTP/REST, AMQP, Kafka.

See docs/glossary.md for full table + conversion scenarios + "above conversion".