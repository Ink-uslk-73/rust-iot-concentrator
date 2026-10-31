# PLATE-C (LIFE) — скоростные каналы, DAS/JBOD, облака

**Статус:** спецификация.
**Связанные:** [ADR-0006](../decisions/ADR-0006-satellite-sovereignty.md), [ADR-0018](../decisions/ADR-0018-wrx90e-platform.md), [pcie-topology.md](pcie-topology.md).

## Роль

PLATE-C (LIFE, ~1000 мс) — скоростные каналы (оптика, WiFi), DAS/JBOD, бэки,
облака/озёра данных, Gateway, маршрутизатор.

**Отдельный компьютер** (4U) в стойке, WRX90E. Автономная перезагрузка, смена ПО, диагностика.

## Платформа

| Параметр | Значение |
|---|---|
| CPU | AMD Threadripper PRO (sTR5, WRX90E) |
| Память | 8× DDR5 R-DIMM (до 2TB) |
| PCIe | 7× PCIe 5.0 x16 (6× x16 + 1× x8) |
| BMC | AST2600 + 1G LAN (автономная диагностика) |
| Сеть | 2× 10G (Intel X710-AT2) |
| Питание | свой БП → PCIe (резерв) → ИБП |

## Каналы (LIFE, 1000 мс)

- Оптика, WiFi (скоростные каналы)
- DAS/JBOD (внешние массивы)
- Бэки, облака/озёра данных
- Gateway, маршрутизатор

## Данные

- **alive.PLATE-C** — «компьютер жив», раз в 1 с.
- **store & forward**: Raw локально (CBOR, 10 МБ, 24 ч), досылка в CORE по PCIe P2P.
- **P2P**: Alveo ×2 на CPU1 (FPGA-to-FPGA), Alveo ↔ NVMe.

## Открытые вопросы

- [ ] Оптика: 10G/25G/100G (какая модель)?
- [ ] WiFi: какой модуль (WiFi 6/6E/7)?
- [ ] DAS/JBOD: сколько дисков, какой интерфейс (SAS/SATA/NVMe)?
- [ ] Бэки/облака: протокол (S3, NFS, CIFS)?
