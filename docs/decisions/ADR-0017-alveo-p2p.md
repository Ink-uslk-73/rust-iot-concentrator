# ADR-0017: Alveo P2P (ReBAR/XRT)

**Статус:** accepted
**Дата:** 2026-10-31
**Связанные:** [ADR-0006](ADR-0006-satellite-sovereignty.md), [alveo-p2p.md](../system/alveo-p2p.md).

## Контекст

Alveo P2P — отдельный тип P2P: FPGA (PLATE-B/C) ↔ NVMe/Alveo внутри сателлита,
через PCIe ReBAR (local switch). Не концентратор (xGMI, ADR-0006). Требует
BIOS (Above 4G, Large BAR, IO Memory base 12T), ACS off, IOMMU off (риск изоляции).

## Решение

- PeerLink::FpgaP2p (Alveo/ReBAR/XRT) — опционально (P3.31).
- StoreForward — P2P drain (NVMe/Alveo) — P3.31.
- BIOS: 12T / Above 4G / ACS off — P3.32.
- P2P только на одном PCIe switch (local), cross-socket — через xGMI (ADR-0006).

## Следствия

+ Скорость: P2P внутри сателлита, без RAM хоста.
+ Изоляция: P2P не выходит за пределы сателлита.
- ACS/IOMMU off — риск изоляции (компромисс).
- Требует BIOS-настроек (не «из коробки»).