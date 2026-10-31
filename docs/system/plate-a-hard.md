# PLATE-A (HARD) — полевые протоколы + AIG-502

**Статус:** спецификация.
**Связанные:** [ADR-0006](../decisions/ADR-0006-satellite-sovereignty.md), [ADR-0018](../decisions/ADR-0018-wrx90e-platform.md), [pcie-topology.md](pcie-topology.md).

## Роль

PLATE-A (HARD, ~10 мс) — полевые протоколы и AIG-502:
Modbus RTU/TCP, PROFINET, EtherNet/IP, CAN, OPC UA. AIG-502 — эмулятор (продакшен, железо не готово).

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

## Протоколы (HARD, 10 мс)

- Modbus RTU / Modbus TCP
- PROFINET, EtherNet/IP, CAN
- OPC UA
- AIG-502 (Modbus RTU, эмулятор)

## Данные

- **alive.PLATE-A** — «компьютер жив», раз в 1 с.
- **store & forward**: Raw локально (CBOR, 10 МБ, 24 ч), досылка в CORE по PCIe P2P.
- **P2P**: Alveo ×2 на CPU1 (FPGA-to-FPGA), Alveo ↔ NVMe.

## Открытые вопросы

- [ ] Кол-во Modbus-каналов (RTU/TCP)?
- [ ] AIG-502: эмулятор → реальное железо (когда?)?
- [ ] Тип watchdog / RTC (CH32V003, DS3231SN)?
- [ ] Точные слоты CPU0/CPU1 (по manual WRX90E)?
