# PCIe топология (WRX90E-SAGE SE)

**Статус:** accepted
**Связанные:** [ADR-0014](../decisions/ADR-0014-pcie-ntb-backplane.md), [ADR-0017](../decisions/ADR-0017-alveo-p2p.md), [ADR-0018](../decisions/ADR-0018-wrx90e-platform.md).

**Платформа:** ASUS Pro WS WRX90E-SAGE SE, **1× sTR5** (Threadripper PRO 7000/9000 WX).
**Не 2P**: 7 слотов PCIe 5.0 x16 от **одного** CPU (6× x16 + 1× x8).

## Слоты PCIe 5.0

| Слот | Режим | Примечание |
|---|---|---|
| PCIe 5.0 x16 #1–6 | x16 | 6 полноценных слотов |
| PCIe 5.0 x16 #7 | x8 | один слот в режиме x8 |

Точная таблица bifurcation — по manual WRX90E: https://www.asus.com/support/FAQ/1037507/

## Распределение по сокетам (архитектурное решение)

- **CPU0**: CORE (Redis/Raw/TimescaleDB), Dispatcher, API, BMC — система.
- **CPU1**: Alveo ×2 (FPGA-to-FPGA P2P), конвертация протоколов, NVMe (Raw).
- **Причина**: P2P только на одном switch (CPU1), разделение ролей, апгрейд без остановки CORE.

## Alveo (FPGA) — CPU1, оба на одном switch

- **PLATE-B/C (WRX90E)**: оба Alveo — **на CPU1** (один PCIe switch), P2P через ReBAR.
- **Назначение**: конвертация протоколов, свитчи, расширение базы протоколов.
- **P2P**: FPGA-to-FPGA (Alveo ↔ Alveo), Alveo ↔ NVMe — local switch, без xGMI.
- **ReBAR**: ≥64GB BAR, XRT (`xbutil`/`xrt-smi`), `p2p --enable` + warm reboot.
- **BIOS**: Above 4G, Large BAR, IO Memory base 12T, ACS off, IOMMU off (риск изоляции).

## M.2 / SlimSAS (без SATA)

- **4× M.2 PCIe 5.0 x4** — NVMe (Raw, Redis, TimescaleDB).
- **2× SlimSAS** (PCIe 4.0 x4) — **только NVMe** (SATA не поддерживается). Для SATA M.2 — обычный SATA-порт (4× SATA 6Gb/s на плате) или USB-адаптер.
- SATA **не используется для основных накопителей** (Raw/Redis/TimescaleDB — NVMe M.2/SlimSAS). SATA-порт (4× 6Gb/s) — только для загрузочного диска / логов, если нужно.

## Питание

- **2× 8-pin CPU + 2× 8-pin PCIe power** — для карт >75W (Alveo U30 150W).
- **Трёхуровневое**: свой БП → PCIe (резерв) → ИБП (последний рубеж).

## Открытые вопросы

- [ ] Точная таблица bifurcation (по manual WRX90E).
- [ ] Какие слоты от CPU0, какие от CPU1 (по manual).
- [ ] Alveo U25 или U30 (75W vs 150W)?
- [ ] NVMe: M.2 или SlimSAS (какие модели)?
