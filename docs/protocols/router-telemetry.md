# Router Telemetry (SNMP, NetFlow/IPFIX, Syslog, gNMI, sFlow)

**Статус:** контекст, реальный Router WI (не эмуляция).
**Связанные:** [ADR-0010](../decisions/ADR-0010-router-telemetry.md), [telemetry.md](../api/telemetry.md), [dcu.md](dcu.md), [glossary](../glossary.md).

## Протоколы сбора

| Протокол | Модель | Что даёт |
|---|---|---|
| SNMP v2c/v3 | Poll (pull) + Traps/Informs | Инвентарь, интерфейсы, hardware-здоровье, события |
| NetFlow v9 / IPFIX | Export (push) | 5-кортеж, bytes/packets, timestamps, BGP/AS-path |
| sFlow | Sampled export | Выборка заголовков + счётчики |
| gNMI / gRPC | Subscribe (push) | YANG-модели, до sub-second, ON_CHANGE |
| NETCONF / RESTCONF | Request-response | Конфиг + state (YANG) |
| Syslog | Push | Текстовые события (up/down, auth, config) |
| YANG-Push | Push | Уведомления об изменениях |

## Trap vs Inform (UDP 162)

- **Trap** — «fire and forget», без подтверждения; потеря — безвозвратно;
- **Inform** — с подтверждением (`GetResponse`), retries до лимита; для критичных (питание, link down);
- Для массовых/некритичных — **Trap** (дешевле).

## Данные

### Интерфейсы
- Счётчики: InPkts, OutPkts, InOctets, OutOctets, Unicast/Multicast/Broadcast, Discards, Errors;
- Состояние: oper-status, admin-status, speed, duplex, MTU, MAC;
- QoS: queue occupancy, drops per queue, WRED counters.

### Платформа / Hardware
- CPU, memory (RAM/flash), temperature, fans, PSU;
- ASIC/pipeline errors, hardware timestamps.

### Маршрутизация
- BGP: peer state, prefixes, AS-path, RIB;
- OSPF/IS-IS: adjacency, LSA/LSDB;
- MPLS: LSP state, FEC, tunnel counters;
- VRF/VPN: instance state, route leaking.

### Соседи
- LLDP: neighbor name, port, capabilities;
- CDP (Cisco).

### Трафик (Flow)
- 5-кортеж (src/dst IP, src/dst port, protocol);
- Bytes, packets, first/last seen;
- Ingress/egress interface, AS-path, BGP community, QoS class.

### Безопасность
- ACL hits/drops;
- IPS/firewall events;
- Authentication failures (SNMPv3, SSH, TACACS+).

### События / Syslog
- Link up/down;
- Config change (who/what/when);
- Process crashes, watchdog resets;
- NTP sync, clock changes.

## SNMP MIB

Vendor-зависимые: `MIKROTIK-MIB`, `CISCO-*`, `INHAND-*`. Плоские OIDs, минуты между опросами.

## Scheduled Tasks (Windows) / cron (Unix)

Polling SNMP по расписанию (раз в 1–5 мин).

## Поиск смарт-устройств (Ethernet, USB, Wi-Fi)

LLDP, CDP, ARP, mDNS — обнаружение соседей.

## Collector на `frontier`

| Протокол | Порт |
|---|---|
| SNMP traps/informs | UDP **162** |
| NetFlow/IPFIX | UDP **2055** |
| sFlow | UDP **6343** |
| Syslog | UDP **514** |
| gNMI | TCP **57400** (TLS) |

## Ссылки

- [ADR-0010: Router telemetry](../decisions/ADR-0010-router-telemetry.md) — решение, P2.24;
- [telemetry.md](../api/telemetry.md) — контракт для планшета (REST + SSE);
- [dcu.md](dcu.md) — DCU как частный случай фронтира;
- [glossary](../glossary.md) — термины `SNMP`, `NetFlow/IPFIX`, `sFlow`, `gNMI`, `Syslog`;
- [TODO.md](../../TODO.md) — P2.24 (Router WI telemetry), P2.25 (SSE).

