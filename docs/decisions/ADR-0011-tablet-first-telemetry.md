# ADR-0011: Планшет как первый потребитель телеметрии (REST + SSE, mTLS)

**Статус:** принято
**Дата:** 2026-10-30

## Контекст

Планшет — **браузер** (**`Frontier Cockpit`**, `http://127.0.0.1:1967/`). **Первый** **потребитель** **телеметрии**; **нативное** **наложение** — **потом** (**тот** **же** **API**).

## Решение

- **REST** — `GET /api/v1/telemetry`, `/events`, `/satellites`, `/metrics`;
- **SSE** — `GET /api/v1/telemetry/stream` (**real-time**, `KeepAlive 15s`, `Last-Event-ID`);
- **mTLS** — клиент **планшета** (**браузер**), **CN** = `tablet-*` (**certs/**);
- **Нативное** **наложение** — **потом** (**тот** **же** **API**, **без** **изменений**).

## Почему SSE, а не WebSocket

| Критерий | SSE | WebSocket |
|---|---|---|
| Направление | server → client | bidirectional |
| Auto-reconnect | встроен (`EventSource`) | вручную |
| Proxy | стандартный HTTP | upgrade (может ломаться) |
| Бинарные | нет | да |
| Сложность | проще | сложнее |

**Для** **телеметрии** (**server → client**, **JSON**, **auto-reconnect**) — **SSE** **идеально** .

## Обоснование

- **Планшет** **уже** **есть** (**браузер**, **`Frontier Cockpit`**);
- **SSE** — **проще** **WebSocket** (**HTTP**, **auto-reconnect**);
- **mTLS** — **безопасность** (**certs/**, **`docs/api/`**);
- **Нативное** — **потом** (**API** **не** **меняется**).

## Что НЕ входит

- **SSE** **реализация** — **P2.25**, **TBD**;
- **Нативное** **наложение** — **P4**, **future**.

## Последствия

- `docs/api/telemetry.md` — **контракт** (**REST + SSE**);
- `TODO.md` — `P2.25` (SSE);
- `src/api/telemetry.rs` — **SSE** **handler** (P2.25).

## Ссылки

- [telemetry.md](../api/telemetry.md) — контракт;
- [ADR-0010: Router WI telemetry](ADR-0010-router-telemetry.md) — источник;
- [TODO.md](../../TODO.md) — P2.25 (SSE).
