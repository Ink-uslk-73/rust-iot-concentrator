# NUMA + tokio (проблема и как проверить)

**Статус:** accepted (P3.28)
**Связанные:** [domain-priority.md](domain-priority.md), [ADR-0017](../decisions/ADR-0017-alveo-p2p.md), [TODO](../../TODO.md).

## Проблема

`sched_setaffinity(0, ...)` **действует** **только** **на** **текущий** **поток** (`main`). **`tokio::main(flavor = "multi_thread")`** **создаёт** **N** **worker**-**потоков** (**num_cpus**), **которые**:

- **не** **наследуют** **affinity**;
- **не** **наследуют** **`SCHED_FIFO`**;
- **не** **наследуют** **`mlockall`**/**mempolicy**.

**Результат:** **HARD-задачи** **мигрируют** **по** **ядрам**/**NUMA** — **RT-гарантии** **отсутствуют**.

## Решение (ADR, TBD)

- **HARD** — **dedicated** **OS**-**thread** (`std::thread::Builder` + **`block_on`**), **`sched_setaffinity`** **на** **этом** **потоке**;

Внутри гостя — `numactl -H`, `cargo test --lib`, `sched_setaffinity` **работает**.

**2. `numactl` на любом Linux** — **`numactl --cpunodebind=0 --membind=0 ./frontier`**, `likwid-pin -c S0:0-3`.

**3. HIL (2P EPYC)** — **`numactl -H`** + **`/proc/<tid>/stat`** поле 39 (core id) + `numa_maps`.

## В проекте

- **`src/app/domains.rs`** — `run_domain_with_sem(..., Duration::from_millis(10/100/1000), ...)` — **`tokio::spawn`** (не dedicated);
- **`src/core/balancer.rs`** — `last_auto` (manual override);
- **TODO** — **P3.28** (NUMA + tokio);
- **PLATE-A/B/C** — **NUMA** **0-7/72-95/8-31** (см. domain-priority.md).

## Ссылки

- [domain-priority.md](domain-priority.md) — HARD/SOFT/LIFE;
- [ADR-0017](../decisions/ADR-0017-alveo-p2p.md) — Alveo P2P;
- [TODO.md](../../TODO.md) — P3.28 (NUMA + tokio).

## Открытые вопросы

- HARD dedicated OS-thread — один на домен или один на PLATE?
- sched_setaffinity — какие ядра (0-7 / 72-95 / 8-31)?
- mlockall — весь процесс или только HARD-поток?
- NPS1 vs NPS4 — финальное решение для 2x EPYC 7002?
- Проверка /proc/<tid>/stat поле 39 — автоматизировать в тесте?