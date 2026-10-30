# Приоритетная обработка: HARD / SOFT / LIFE

**Статус:** принято
**Дата:** 2026-10-30
**Связанные:** [ADR-0008](../decisions/ADR-0008-frontier-term.md), [ADR-0015](../decisions/ADR-0015-combination-innovation.md), [glossary](../glossary.md).

## Контекст

Концентратор — **три** **уровня** **приоритета**: **HARD** (10 мс), **SOFT** (100 мс), **LIFE** (1000 мс). **Нужна** **детальная** **спецификация** (**не** **только** **строка** **в** **README**).

## Уровни

| Уровень | Период | Назначение | Missed tick | NUMA |
|---|---|---|---|---|
| **HARD** | 10 мс | Критические операции: Modbus RTU, тайминги, радиоканал | **Delay/Skip** (не копить) | node 0, cores 0-7 |
| **SOFT** | 100 мс | Маршрутизация, состояния, метрики | **Skip** | node 1, cores 72-95 |
| **LIFE** | 1000 мс | Логирование, heartbeat, статистика, кокпит | **Burst** (допустим) | node 0, cores 8-31 |

## Реализация

- **Три** **независимых** **цикла** (`tokio::time::interval`), **изоляция** **через** **`Supervisor::spawn`** (`src/supervisor.rs`);
- **`VmSemaphore`** (`src/vm/state.rs`) — **изоляция** **доменов**;
- **`run_domain_with_sem`** (`src/vm/run_domain_with_sem`) — **обёртка** **с** **семофором**;
- **`FrontierState`** (**ADR-0008**) — **связь**:
  - **HARD** — `Wild` → `Enriched` (Modbus, нормализация);
  - **SOFT** — `Aggregated` → `Structured` (окна, схема);
  - **LIFE** — `Clustered` → `Patterned` → `Explored` (DBSCAN/spectral, motifs, облака).

## Backlog (политика)

- **HARD** — **drop** (не **копить**, **fresh** **важнее**);
- **SOFT** — **queue** ( **bounded** **mpsc**, **drop** **oldest**);
- **LIFE** — **merge** (**агрегация** **в** **TimescaleDB**).

## Изоляция

- **Отдельные** **ядра** (**taskset**/**cgroup**);
- **NUMA**: **HARD** — node 0 (0-7), **SOFT** — node 1 (72-95), **LIFE** — node 0 (8-31);
- **`numa_foreign == 0`** (**health-check**).

## Что НЕ входит

- **Preemption** (**HARD** **прерывает** **SOFT**) — **нет**, **только** **параллельность**;
- **Единый** **event loop** — **нет**, **три** **независимых** **цикла**.

## Ссылки

- [ADR-0008: Фронтир](../decisions/ADR-0008-frontier-term.md) — 7 состояний;
- [ADR-0015: Комбинация](../decisions/ADR-0015-combination-innovation.md) — новация;
- [glossary](../glossary.md) — термины `HARD`, `SOFT`, `LIFE`;
- [src/vm/run_domain_with_sem](../../src/vm/run_domain_with_sem) — реализация;
- [TODO.md](../../TODO.md) — P3.24 (domain priority).
