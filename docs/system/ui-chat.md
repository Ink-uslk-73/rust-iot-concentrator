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
## API (P3.43)

- `POST /api/v1/chat` — body `{"text": "...", "tab": "engineering"|"audit"}`; RBAC (write): `X-Role: engineer`/`onduty` (200), `auditor`/`llm` (403), без заголовка (401).
- `GET /api/v1/chat/stream` — SSE, event `chat` (JSON `ChatMsg`); RBAC (read): все роли (engineer/auditor/onduty/llm).
- Вкладки: `tab` в `ChatMsg` (engineering/audit); UI фильтрует по `tab` (SSE event).

## Канал (broadcast)

- `chat_tx: broadcast::Sender<ChatMsg>` — **отдельно** от `telemetry_tx` (Оккам: не смешивать телеметрию и чат).
- `POST` → `chat_tx.send(ChatMsg)`; `GET /chat/stream` → `BroadcastStream` (фильтр по `tab` — в UI).

## RBAC (роли)

| Роль | read (SSE) | write (POST) |
|---|---|---|
| engineer | ✅ | ✅ |
| onduty | ✅ | ✅ |
| auditor | ✅ | ❌ (403) |
| llm | ✅ | ❌ (403) |
| без заголовка | ✅ (read) | ❌ (401) |

## ChatMsg (Rust)

- `ChatMsg { text: String, tab: String, role: String, ts: u64 }` (Serialize).