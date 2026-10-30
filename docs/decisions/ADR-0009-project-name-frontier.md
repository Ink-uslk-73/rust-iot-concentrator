# ADR-0009: Имя проекта и главной программы — Frontier

**Статус:** принято
**Дата:** 2026-10-30

## Контекст

У проекта **нет имени** и **нет имени главной программы** в UI/CLI/API.
`rust-iot-concentrator` — **техническое** (repo, crate), не отражает сущность.
Нужно имя, которое отражает **роль** (граница освоенного/дикого, фронтир анализа).

## Решение

**Имя проекта:** `Frontier` (вместо `rust-iot-concentrator` — repo name сохраняется).

| Слой | Имя |
|---|---|
| Repo | `rust-iot-concentrator` (сохраняется) |
| Crate (`Cargo.toml` `[package] name`) | `frontier` |
| Bin (`[[bin]] name`) | `frontier` |
| UI-заголовок | `Frontier` |
| API health | `GET /api/v1/health` → `frontier: ok` |
| Логи | `frontier: ...` (вместо `rust_iot_concentrator: ...`) |
| CLI | `frontier --config ...` |

## Обоснование

- **Отражает сущность**: `Frontier` — граница освоенного/дикого, фронтир анализа;
- **Совпадает с термином** (`ADR-0008`): термин `Фронтир` (7 состояний, «улитка Улама»);
- **Короткое**, **произносимое**, **уникальное**;
- **Не привязано к технологии** (`rust-` в имени — техническое).

## Что НЕ входит

- **Переименование repo** (`rust-iot-concentrator` — остаётся на GitHub);
- **Переименование `src/`** (структура не меняется);
- **`Cargo.lock`** (обновится автоматически при `cargo build`).

## Последствия

- `Cargo.toml` — `[package] name = "frontier"`, `[[bin]] name = "frontier"`;
- `README.md` — заголовок `# Frontier` (repo name — в описании);
- `UI` — заголовок `Frontier` (`index.html` `<title>`, `Dashboard.svelte`);
- `API` — `GET /api/v1/health` → `frontier: ok`;
- `docs/glossary.md` — `Frontier` (уже добавлен);
- `TODO.md` — упоминания `frontier` (bin, UI, API).

## Ссылки

- [ADR-0008: Фронтир](../decisions/ADR-0008-frontier-term.md) — термин;
- [README.md](../../README.md) — имя проекта;
- [Cargo.toml](../../Cargo.toml) — crate/bin `frontier`;
- [docs/glossary.md](../glossary.md) — термин `Frontier`.

