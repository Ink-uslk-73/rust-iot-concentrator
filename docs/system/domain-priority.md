# Приоритетная обработка: HARD / SOFT / LIFE

**Статус:** принято
**Дата:** 2026-10-30
**Связанные:** [ADR-0008](../decisions/ADR-0008-frontier-term.md), [ADR-0015](../decisions/ADR-0015-combination-innovation.md), [glossary](../glossary.md).

## Контекст

Концентратор — **три** **уровня** **приоритета**: **HARD** (10 мс), **SOFT** (100 мс), **LIFE** (1000 мс). **Нужна** **детальная** **спецификация** (**не** **только** **строка** **в** **README**).

## Уровни

| Уровень | Период | Назначение | Missed tick | NUMA |
|---|---|---|---|---|
| **HARD** | 10 мс | Критические операции: Modbus RTU, тайминги, радиоканал | **Skip** (`vm/mod.rs:48`) | node 0, cores 0-7 |
| **SOFT** | 100 мс | Маршрутизация, состояния, метрики | **Skip** (`vm/mod.rs:48`) | node 1, cores 72-95 |
| **LIFE** | 1000 мс | Логирование, heartbeat, статистика, кокпит | **Skip** (`vm/mod.rs:48`) | node 0, cores 8-31 |

## Реализация

- **Три** **независимых** **цикла** (`tokio::spawn` + `tokio::time::interval`), **`Skip`** для всех (`vm/mod.rs:47-48`);
- **`VmSemaphore`** (`src/vm/state.rs`) — **изоляция** **доменов**;
- **`run_domain_with_sem(domain, tick, ...)`** (`src/vm/mod.rs:202`) — `tick: Duration`, `MissedTickBehavior::Skip`;
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
## Факты (grep src/)

| Что | Где | Значение |
|---|---|---|
| **Три цикла** | `src/app/domains.rs:47,52,57` | `run_domain_with_sem(..., Duration::from_millis(10/100/1000), ...)` |
| **Missed tick** | `src/vm/mod.rs:48` | `MissedTickBehavior::Skip` (все домены) |
| **Priority** | `src/core/mode_switch.rs:10-14` | `enum Priority { Hard, Soft, Life }`, `0/1/2` |
| **Dispatcher** | `src/core/dispatcher.rs:102-107` | `Priority::Hard → DomainKind::Hard`, ... |
| **mode_check** | `src/core/dispatcher.rs:149-150` | `interval(100ms)` + `Skip` |
| **hard_pool SLA** | `src/hard_pool/worker.rs:43` | `if dt > 10ms → sla_breaches++` |
| **hard_pool scheduler** | `src/hard_pool/scheduler.rs:70` | `pool.submit(Duration::from_millis(10), ...)` |
| **CPU pinning** | — | **нет** (`taskset`/`cgroup`/`affinity`) → **P3.25** TBD |

**Вывод:** `Skip` для всех трёх доменов (не `Delay`/`Burst`). `Duration::from_millis(10/100/1000)` — в `app/domains.rs`. `hard_pool` — пул с SLA 10 мс, не просто цикл.