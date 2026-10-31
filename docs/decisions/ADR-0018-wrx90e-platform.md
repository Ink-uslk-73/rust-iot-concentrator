# ADR-0018: WRX90E-SAGE SE как платформа сателлитов

**Статус:** accepted
**Дата:** 2026-11-01
**Связанные:** [ADR-0001](ADR-0001-carrier-board-for-atx.md), [ADR-0006](ADR-0006-satellite-sovereignty.md), [ADR-0017](ADR-0017-alveo-p2p.md), [carrier-board.md](../system/carrier-board.md).

## Контекст

X870E-Creator WiFi (AM5) не подходит: 2× PCIe 5.0 x16 (x8/x8), нет BMC,
4× DDR5 (256GB max). Для сателлитов (PLATE-A/B/C) нужно: 7 слотов PCIe 5.0,
BMC (автономная диагностика), 8× R-DIMM (до 2TB), 2× 10Gb LAN, 2× 8-pin PCIe power.

## Решение

Целевая платформа сателлитов — ASUS Pro WS WRX90E-SAGE SE (sTR5, Threadripper PRO):
7× PCIe 5.0 x16, BMC AST2600 + 1G LAN, 8× DDR5 R-DIMM (2TB), 2× 10Gb (Intel X710-AT2),
питание (16×2)+3+3+3, 2× 8-pin PCIe power.

## Следствия

+ 7 слотов — под PLATE-A/B/C, AIG-502, Alveo, NVMe, резерв.
+ BMC — автономная перезагрузка/диагностика сателлитов без ОС.
+ 8× R-DIMM — 2TB, под Raw CBOR / store & forward.
+ 2× 10Gb — uplink (MQTT/HTTP/API) + резерв.
- Дороже X870E; требует Threadripper PRO (sTR5), R-DIMM.
- PCIe power 2× 8-pin — учесть в бюджете (возможно, 1600–2000 Вт).

## Горизонт

- **Продакшн**: 2029 (WRX90E проверен, SP8 зрелый).
- **Демо**: раньше, на эмуляторах (AIG-502, умный дом, каналы).
- **Кастомизация**: RAID, Nomad, число сателлитов — по требованию заказчика.
