# BOM (без SATA)

## Основа

| Компонент | Кол-во | Примечание |
|---|---|---|
| ASUS ProArt X870E-Creator WiFi | 1 | reference module, ATX 30.5 × 24.4 |
| AMD Ryzen 9 9000 (AM5) | 1 | TDP ~250 Вт sustained |
| DDR5 ECC UDIMM | 4 | до 192 GB (manual), ECC подтверждён |
| NVMe M.2 (PCIe 5.0 / 4.0) | до 4 | M.2_1/2 — PCIe 5.0, M.2_3/4 — PCIe 4.0 |

## Несущая

| Компонент | Кол-во | Примечание |
|---|---|---|
| PCB несущей | 1 | E-ATX или custom (TODO) |
| Разъём 24-pin ATX | 1 | от БП 1200 Вт |
| Разъём 2 × 8-pin CPU (EPS) | 1 пара | ProCool II |
| 12VHPWR (600 Вт) | 1 | для PCIEX16(G5)_1 |
| 6+2-pin PCIe ×2 | 1 | для PCIEX16(G5)_2 |
| DC-DC 12V→5V | 1 | Modbus, RTC |
| DC-DC 12V→3.3V | 1 | watchdog, логика |
| DC-DC 12V→3.3V (изолир.) | 1 | RS-485/232 развязка |
| Modbus RS-485 трансивер | TODO | кол-во каналов |
| Modbus RS-232 трансивер | TODO | 1–2 канала |
| Watchdog | 1 | MAX6369? (TODO) |
| RTC | 1 | DS3231? (TODO) |
| Разъёмы полевых шин | TODO | клеммники, DB9, RJ-45 ind. |

## Исключено

- **SATA** — не используем.
- **P2P / ACS / xGMI** — не входит в системотехнику сателлита.
- **PCIe P2P** — у концентратора.

## Открытые вопросы

- [ ] Корпус (4U / industrial / custom)?
- [ ] Габариты несущей (E-ATX / custom)?
- [ ] Кол-во Modbus-каналов?
- [ ] Тип watchdog / RTC?

