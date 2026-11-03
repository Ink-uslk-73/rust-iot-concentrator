# UI: Чат ЦУ (P3.43)

**Статус:** accepted (P3.43).
**Связанные:** [ui-engineering.md](ui-engineering.md), [ui-audit.md](ui-audit.md), [satellite-vm.md](satellite-vm.md).

## Роль

Общий чат ЦУ (концентратор — ядро, бригада — для мини-ЦОДа).
Вкладки: Engineering (read-write, инженер), Audit (read-only, аудитор, LLM).
Почему вкладки: шум мешает основной деятельности.

## Компоненты

- UI — CORE (axum + SSE, веб, IPv6 ULA);
- LLM — локальный (Ollama/llama.cpp) в AUDIT-VM (изоляция, не утечка);
- RBAC — CORE (роли: инженер, аудитор, дежурный, LLM).

## Вкладки

| Вкладка | Роль | Права |
|---|---|---|
| Engineering | Инженер (проектирует, настраивает, диагностирует) | read-write |
| Audit | Аудитор (доказательства, отчёт) | read-only (LLM) |

## Открытые вопросы

- [ ] LLM: Ollama или llama.cpp (локально в AUDIT-VM)?
- [ ] RBAC: роли (инженер, аудитор, дежурный) — финально?
- [ ] Чат: один поток / вкладки — финально?
