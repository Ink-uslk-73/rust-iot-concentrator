# ADR-0008: Фронтир — термин (пространственная и аналитическая природа)

**Статус:** принято
**Дата:** 2026-10-30

## Контекст

Термин **Фронтир** (Frontier) используется в проекте для описания границы между
«освоенной территорией» (облако, HES/MDMS, центральные серверы) и «дикой землёй
данных» (сенсоры, счётчики, IoT-устройства). В отличие от метафоры, **Фронтир** —
это **термин**, который будет отражён в коде (`src/core/frontier.rs`).

## Решение

**Фронтир** — двойственная природа:

1. **Пространственная** (освоенное vs дикое):
   - `Wild` (Dew) — автономные устройства, сырые данные;
   - `Frontier` (Fog) — концентратор, агрегация, анализ;
   - `Explored` (Cloud) — HES/MDMS, освоено.
2. **Аналитическая** (шум vs смысл):
   - `Raw` → `Enriched` → `Aggregated` → `Structured` → `Clustered` → `Patterned`.

**Аналогия — «улитка Улама»** (Ulam spiral): самоподобная структура, каждый виток —
масштаб (частота, окно), траектория — последовательность состояний. `Wavelet`-анализ
и `motif discovery` видят «слои» (витки).

## FrontierState (7 состояний)

| Состояние | Что | Связь с Dew/Fog/Edge |
|---|---|---|
| `Wild` | сырые данные, автономные устройства | Dew |
| `Enriched` | парсинг, нормализация | Fog |
| `Aggregated` | окна, метрики | Fog |
| `Structured` | схема данных | Fog |
| `Clustered` | DBSCAN / spectral | Fog |
| `Patterned` | motifs / wavelets | Fog |
| `Explored` | освоено, HES/MDMS | Cloud |

**Контракт** (`src/core/frontier.rs`, TBD):

```rust
pub enum FrontierState {
    Wild, Enriched, Aggregated, Structured, Clustered, Patterned, Explored,
}

pub struct Scale { window_ms: u64, level: u8 }  // виток улитки

pub struct Frontier {
    state: FrontierState,
    devices: Vec<DeviceId>,
    protocols: Vec<ProtocolId>,
    scale: Scale,
    depth: u8,   // уровень вложенности (фрактал)
}
```

## Связь с проектом

- **PLATE-A (Hard)** = AIG-502 — `Wild` → `Enriched` (Modbus, нормализация);
- **PLATE-B (Soft)** — `Frontier` (вычисления, «дружба» с PLATE-C);
- **PLATE-C (Life)** — `Clustered` / `Patterned` (SmartNIC/AI, СУБД, облака);
- **Митрополия** (концентратор) — `Explored` (HES/MDMS).

## Что НЕ входит

- **Кластеризация в коде** — `DBSCAN`, `spectral`, `GMM`, `K-Means` — **позже**,
  в `src/analytics/` (после TimescaleDB, P3).
- **Vector DB** (`Milvus`, `Qdrant`) — **позже** (ML, P4).

## Последствия

- `src/core/frontier.rs` — создать (TBD, после docs).
- `ProtocolAdapter` — **не** переименовываем в `FrontierAdapter` (адаптер — часть фронтира,
  но не весь фронтир).
- `docs/glossary.md` — `## Концепции` — `Фронтир` (7 состояний, «улитка Улама»).
- `docs/protocols/dcu.md` — DCU как **частный случай** фронтира.
- `TODO.md` — `P2` `src/core/frontier.rs`, `P3` аналитика.

## Ссылки

- [DLMS/COSEM adapter](ADR-0007-dlms-cosem-adapter.md) — протокол на фронтире;
- [Satellite sovereignty](ADR-0006-satellite-sovereignty.md) — митрополия/протектораты;
- [docs/glossary.md](../glossary.md) — термин «Фронтир»;
- [docs/protocols/dcu.md](../protocols/dcu.md) — DCU как частный случай фронтира;
- [TODO.md](../../TODO.md) — P2 `src/core/frontier.rs`, P3 аналитика.

