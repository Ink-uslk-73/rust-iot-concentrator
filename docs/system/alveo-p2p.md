# AMD Alveo P2P (ReBAR, XRT, BIOS) — сателлит PLATE-B/C

**Статус:** accepted
**Связанные:** [ADR-0017](../decisions/ADR-0017-alveo-p2p.md), [ADR-0006](../decisions/ADR-0006-satellite-sovereignty.md), [pcie-topology.md](pcie-topology.md), [glossary](../glossary.md).

## Контекст

**Alveo P2P** — **отдельный** **тип** **P2P**: **FPGA** (**PLATE-B/C**) ↔ **NVMe**/**Alveo** **внутри** **сателлита**, **через** **PCIe** **ReBAR** (**local** **switch**). **Не** **концентратор** (**xGMI**, **ADR-0006**).

## Alveo P2P

| Параметр | Значение |
|---|---|
| **Механизм** | **PCIe** **P2P** **через** **Resizable BAR** (**ReBAR**), **без** **RAM** **хоста** |
| **Инструментарий** | **Vitis PCIe platforms**, **XRT runtime** (`xbutil`/`xrt-smi`) |
| **Включение** | `p2p --enable` (**XRT**), **warm reboot** **обязателен** |
| **BIOS** | **Above 4G decoding**, **Large BAR support**, **IO Memory base 12T** (>32T — риск) |
| **ACS** | **ACS off** (**P2P**), **IOMMU** **off** (риск изоляции) |
| **Ограничение** | **P2P** **только** **на** **одном** **PCIe** **switch** |
| **Alveo** **U25 SmartNIC** | **75W** (слот), **FHFL** |
| **Alveo** **U30** | **150W** (8-pin), **FHFL** |
| **FPGA ↔ NVMe** | **P2P** **с** **DMA** (**QDMA**/**XDMA**), **SMART SSD** |

## В проекте

- **PLATE-B/C** — **Alveo** **в** **PCIEX16(G5)_1** (**x16**), **ReBAR** ≥64GB **BAR**;
- **`PeerLink`** — **`FpgaP2p`** (**Alveo**/**ReBAR**/**XRT**) — **опционально** (**P3.31**);
- **`StoreForward`** — **P2P** **drain** (**NVMe**/**Alveo**) — **P3.31**;
- **BIOS** — **12T**/**Above 4G**/**ACS off** (**P3.32**);
- **4U** — **7** **слотов** **FHFL** — **ok** (**carrier-board.md**).

## Ссылки

- [ADR-0017: Alveo P2P](../decisions/ADR-0017-alveo-p2p.md);
- [ADR-0006: Суверенитет](../decisions/ADR-0006-satellite-sovereignty.md) — различие P2P;
- [pcie-topology.md](pcie-topology.md) — PCIe topology;
- [TODO.md](../../TODO.md) — P3.31 (Alveo P2P), P3.32 (BIOS).

## Открытые вопросы

- Alveo U25 или U30 (75W vs 150W)?
- ReBAR >=64GB — какой именно BAR (1x64GB или 2x32GB)?
- PeerLink::FpgaP2p — обязателен или опционален (сейчас «опционально»)?
- P2P только на одном PCIe switch — какой switch (CPU0/CPU1)?
- SMART SSD — какая модель, поддерживает ли P2P?
