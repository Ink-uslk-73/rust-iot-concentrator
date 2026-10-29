# Системотехника сателлита PLATE-3

Сателлит построен на **несущей плате (carrier board)**, на которую устанавливается
стандартная **ATX-совместимая материнская плата** через стойки. Вокруг неё размещается
вся периферия: разъёмы внешнего БП 1200 Вт, DC-DC, Modbus-трансиверы, watchdog, RTC.

## Референсный модуль

За основу взята **ASUS ProArt X870E-Creator WiFi** (AM5, X870E, PCIe 5.0, ATX).
Она задаёт габариты, расположение стоек, слоты PCIe и высоты. Все остальные
модули размещаются **вокруг** неё на нашей несущей.

## Что НЕ входит в системотехнику сателлита

- **PCIe P2P / ACS / xGMI** — это системотехника **концентратора** (см. `958ea03`),
  не сателлита. В сателлите P2P не используется.
- **SATA** — исключён. Используем только M.2 NVMe.

## Документы

- [carrier-board.md](carrier-board.md) — несущая: габариты, разъёмы, DC-DC, Modbus, watchdog
- [power-budget.md](power-budget.md) — бюджет внешнего БП 1200 Вт
- [pcie-topology.md](pcie-topology.md) — слоты PCIe и bifurcation (без ACS/P2P)
- [router-gateway.md](router-gateway.md) — программные роутер/шлюз по доменам A/B/C
- [cooling.md](cooling.md) — охлаждение, ambient 10-35C
- [bill-of-materials.md](bill-of-materials.md) — BOM (без SATA)
- [reference/asus-proart-x870e-creator.md](reference/asus-proart-x870e-creator.md) — выжимка из manual ASUS

