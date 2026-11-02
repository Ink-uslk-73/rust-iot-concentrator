# Системотехника (docs/system)

**Статус:** accepted
**Связанные:** [ADR-0006](../decisions/ADR-0006-satellite-sovereignty.md), [ADR-0018](../decisions/ADR-0018-wrx90e-platform.md).

Оглавление раздела. Описание каждой платы — в отдельных файлах (plate-a/b/c).

## Платформа (несущая)

- [carrier-board.md](carrier-board.md) — несущая, WRX90E (1× sTR5, E-ATX), 4U × N
- [satellite-vm.md](satellite-vm.md) — 4 VM (DB, NET, BACKUP, BOT) + host (CORE)
- [pcie-topology.md](pcie-topology.md) — 7× PCIe 5.0, Alveo CPU1, SlimSAS (NVMe)
- [power-budget.md](power-budget.md) — 1200/1600 Вт, U25/U30
- [cooling.md](cooling.md) — ambient 10–35 °C
- [hot-swap.md](hot-swap.md) — hot-plug не заявлен, U.2/E3.S

## Сателлиты (по отдельным файлам)

- [plate-a-hard.md](plate-a-hard.md) — PLATE-A (HARD): полевые протоколы + AIG-502
- [plate-b-soft.md](plate-b-soft.md) — PLATE-B (SOFT): умный дом (Matter/Zigbee/Thread/BLE)
- [plate-c-life.md](plate-c-life.md) — PLATE-C (LIFE): скоростные каналы, DAS/JBOD, облака

## Данные и сеть

- [domain-priority.md](domain-priority.md) — HARD/SOFT/LIFE
- [router-gateway.md](router-gateway.md) — роутер/шлюз, relay/smart host
- [network-nodes.md](network-nodes.md) — паттерн NetworkNode (router, switch, bridge, gateway, firewall)
- [numa-tokio.md](numa-tokio.md) — NUMA + tokio
- [monitoring.md](monitoring.md) — мониторинг
- [alveo-p2p.md](alveo-p2p.md) — Alveo P2P (ReBAR/XRT)
- [bill-of-materials.md](bill-of-materials.md) — BOM
- [raw-abstraction.md](raw-abstraction.md) — Raw как абстракция (P3.37)

## Архитектурная метафора (см. ADR-0006)

- **Митрополия** — концентратор (EPYC SP8 2P, xGMI): Bus, Dispatcher, API, smart host.
- **Протектораты** — сателлиты PLATE-A/B/C (WRX90E, 1P): почти автономные, свои драйверы/протоколы.
- **Экспорт** — данные/сигналы/телеметрия от сателлита к концентратору.
- **Импорт** — только по запросу (CPU/память/NUMA/БД концентратора).
- **Иерархия**: Cloud → Fog (сателлиты) → Edge (сенсоры) → Dew (автономная работа).

Подробнее: [ADR-0006](../decisions/ADR-0006-satellite-sovereignty.md).
