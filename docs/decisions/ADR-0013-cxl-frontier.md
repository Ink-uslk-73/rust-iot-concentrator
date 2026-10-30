# ADR-0013: CXL / disaggregated (future, обоснование Frontier)

**Статус:** принято (future, P4.12)
**Дата:** 2026-10-30

## Контекст

Frontier — сателлитная архитектура (митрополия + протектораты). CXL 3.0/4.0 — **следующий шаг** (disaggregated / composable infrastructure). **Обоснование** для Frontier, **но не сейчас**.

## Решение

**Зафиксировать** CXL как **future** (P4.12):

| Компонент | Роль |
|---|---|
| **CXL 3.0** | Type 2 (память + вычисления), memory pooling, cache-coherent |
| **CXL 4.0** (2025) | rack-level switching, hot-plug, fabric manager |
| **Disaggregated** | compute blade + memory blade (Dell, HPE, AMD) |
| **Satellite-Born Server** (TianSuan) | 60 × Snapdragon 865, 12 плат, TpE 1.7–2.5× vs Xeon |
| **Fabric Architecture** (CMU) | 166× энергоэффективность, edge/space |

**Связь с Frontier:**
- **Сателлиты** (PLATE-A/B/C) — **уже** **есть**;
- **CXL** — **логичное** **продолжение** (память/вычисления **на** **отдельных** **платах**);
- **Memory pooling** — **разделяемая** **память** **между** **сателлитами** (future).

## Обоснование

- **Сателлитная** **архитектура** **уже** **есть** (PLATE-A/B/C);
- **CXL** — **следующий** **шаг** (**disaggregated**);
- **Но** **не** **сейчас** (**P4.12**, **future**).

## Что НЕ входит

- **Реализация** `src/transport/cxl.rs` — **P4.12**, **future**;
- **CXL hardware** — **не** **сейчас** (**hardware** **позже**).

## Последствия

- `docs/system/cxl.md` — **создать** (P4.12);
- `TODO.md` — `P4.12` (CXL);
- **Frontier** — **обоснован** (**сателлитная** **архитектура**).

## Ссылки

- [ADR-0008: Фронтир](ADR-0008-frontier-term.md) — 7 состояний;
- [ADR-0014: PCIe NTB backplane](ADR-0014-pcie-ntb-backplane.md) — 2 × PEX 8749;
- [pcie-ntb-backplane.md](../system/pcie-ntb-backplane.md) — инженерное решение;
- [TODO.md](../../TODO.md) — P4.12 (CXL future).
