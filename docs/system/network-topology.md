# Топология сети (P3.42)

**Статус:** accepted (P3.42).
**Связанные:** [satellite-vm.md](satellite-vm.md), [network-nodes.md](network-nodes.md), [router-gateway.md](router-gateway.md).

## Принцип: IPv6 always, IPv4 — исключение

- IPv6 — всегда (ULA fd00::/8 внутри MAN);
- IPv4 — только исключение (legacy, АСУ ТП с IPv4-only, AIG-502);
- NAT66 — на шлюзе (NET-VM), переход ULA <-> глобальный IPv6;
- MAN — Unicast P2P (не multicast/broadcast/anycast, кроме служебного IPv6).

## VLAN (тематические подсети)

| VLAN | Диапазон (пример) | Назначение |
|---|---|---|
| 10 | fd00:10::/64 | HARD (PLATE-A): Modbus, OPC UA, PROFINET, CAN |
| 20 | fd00:20::/64 | SOFT (PLATE-B): Matter, Zigbee, Thread, BLE |
| 30 | fd00:30::/64 | LIFE (PLATE-C): оптика, WiFi, DAS/JBOD, облака |
| 40 | fd00:40::/64 | API/UI (планшет, SSE, REST) |
| 50 | fd00:50::/64 | Uplink (MQTT, HTTP, OPC UA) |
| 60 | fd00:60::/64 | Служебный (IPv6, NTP, discovery) |
| 70 | fd00:70::/64 | BOT-VM (автоматизация) |
| 80 | fd00:80::/64 | DB-VM (TimescaleDB) |
| 90 | fd00:90::/64 | AUDIT-VM (аудит, read-only) |

**Фильтры**: только своё и только в свой диапазон. Оверлей (NTP, discovery) — частично (контролируемо).

## NET-VM — пересадочный хаб

- VLAN 10–90, NetworkNode (Router/Switch/Bridge/Gateway/Firewall), шлюз, MAN;
- Шлюз — один (CORE, NET-VM), NAT66, mTLS, smart host (не open relay).

## SOAP — магистраль (DCI)

- SOAP — только оптическая магистраль между концентраторами (изоляция, конфиденциальные данные);
- Трансиверы: 400G ER4 (10–40 км, PAM4), ZR/ZRP+ (40–120 км, когерентный);
- MPO-16 (если 800G): single-mode DR8 (OS2), полярность Type A/B;
- Стандарты: OIF-400ZR, QSFP-DD MSA, IEEE 802.3 (не вендор-lock).

## REST — внутри концентратора

- REST — планшет, UI, API (IPv6 ULA, внутри концентратора);
- SSE — telemetry/events (внутри, IPv6 ULA);
- Магистраль — SOAP (оптика), не через broadcast.

## Протоколы (non-stop vs остальное)

- HARD/SOFT — UTP+QUIC (non-stop, UDP-based, низкая латентность);
- LIFE/API/uplink — TCP (надёжность, порядок, MQTT/HTTP/OPC UA).

## Решённые вопросы

- [x] VLAN count: 9 (10–90) — достаточно (первая редакция).
- [x] IPv4-исключения: всё оборудование по спецификациям поддерживает IPv6 → IPv4 только legacy (если попадётся). АСУ ТП и AIG-502 — IPv6.
- [x] NTP: chrony; discovery: unicast DNS-SD (не mDNS, не broadcast).
- [x] Трансиверы: ER4 (400G, 10–40 км) — для ТОСЭР; DR8 (800G) — если понадобится.

## Открытые вопросы (позже)

- [ ] Фильтры по MAC (подмена оборудования) — дополнить при необходимости.
