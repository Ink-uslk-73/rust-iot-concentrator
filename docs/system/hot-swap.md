# Hot-swap / Hot-plug (несущая и шасси)

**Статус:** accepted
**Связанные:** [carrier-board.md](carrier-board.md), [power-budget.md](power-budget.md), [ADR-0001](../decisions/ADR-0001-carrier-board-for-atx.md), [glossary](../glossary.md).

## Терминология

- **Hot-swap** — замена компонента **без остановки** системы и **без команд подготовки** (БП, вентиляторы, диски в RAID с горячей заменой).
- **Hot-plug** — добавление компонента **без остановки** системы, но с **подготовкой ОС** (USB, новый диск в свободный слот).

## Компоненты несущей

| Компонент | Hot-swap | Примечание |
|---|---|---|
| **БП (CRPS 1+1)** | **Да** | 2 × 1200W, hot-swap разъём, замена без остановки |
| **Диски (M.2)** | **Нет** | M.2 внутри корпуса, замена с выключением (hot-swap **не нужен**) |
| **Память (DDR5)** | **Нет** | x86 не поддерживает hot-swap RAM, замена с выключением |
| **FPGA SmartNIC (PCIe)** | **Нет** | ASUS X870E не заявляет PCIe hot-plug, замена с выключением |
| **RTC DS3231SN / CH32V003** | **Нет** | паяные на несущей |

## Требования к несущей

- **БП**: CRPS hot-swap разъём (2 × 1200W, 1+1), замена без остановки;
- **Диски**: M.2 внутри корпуса, замена с выключением (hot-swap не требуется);
- **Память**: DDR5, замена с выключением (x86);
- **FPGA**: PCIe слот, замена с выключением (ASUS не заявляет hot-plug).

## CXL 3.0/4.0 (future)

CXL 4.0 (2025) добавляет **hot-plug ресурсов** и rack-level switching — **future** (P4.12, [ADR-0013](../decisions/ADR-0013-cxl-frontier.md)).
Сейчас **не** используется (нет CXL-плат).

## Ссылки

- [carrier-board.md](carrier-board.md) — несущая;
- [power-budget.md](power-budget.md) — БП 1200 Вт;
- [ADR-0001](../decisions/ADR-0001-carrier-board-for-atx.md) — несущая;
- [ADR-0013](../decisions/ADR-0013-cxl-frontier.md) — CXL future;
- [glossary](../glossary.md) — термины.
