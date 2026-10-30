# DLMS/COSEM (IEC 62056-5-3 / 6-1 / 6-2)

**Статус:** требование к концентратору, **надстройка** (P2/P3, фоном).
**Связанные:** [ADR-0007](../decisions/ADR-0007-dlms-cosem-adapter.md), [ADR-0008](../decisions/ADR-0008-frontier-term.md), [dcu.md](dcu.md), [glossary](../glossary.md).

## Access Security

| Уровень | Механизм ID | Описание |
|---|---|---|
| No Security | 0 | Без аутентификации; только Public Client (адрес 16) |
| LLS (Low Level Security) | 1 | Пароль в открытом виде в AARQ; **только commissioning** |
| HLS (High Level Security) | 2–7 | Взаимная аутентификация client ↔ server, 4 прохода (challenge-response) |

### HLS-механизмы

| ID | Механизм | Алгоритм |
|---|---|---|
| 2 | HLS (manufacturer-specific) | Зависит от производителя |
| 3 | HLS-MD5 | MD5 (устаревший) |
| 4 | HLS-SHA-1 | SHA-1 (устаревший) |
| 5 | HLS-GMAC | GMAC (AES-CMAC) |
| 6 | HLS-SHA-256 | SHA-256 |
| 7 | HLS-ECDSA | ECDSA (сертификаты, без общего секрета) |

**Наш выбор:** HLS-ECDSA (7) — см. [ADR-0007](../decisions/ADR-0007-dlms-cosem-adapter.md).

## Transport Security

| Тип шифрования | Область | Ключ |
|---|---|---|
| Service-specific | Каждое сообщение | Dedicated key (уникальный на сервис) |
| General | Все сообщения в association | Global key |

**Режимы:** Authentication only (GMAC), Encryption only (AES-GCM), Authentication + Encryption (AES-GCM с тегом).

## Security Suites

| Suite | Auth. Encryption | Digital Signature | Key Agreement | Hash | Key Transport |
|---|---|---|---|---|---|
| 0 | AES-GCM-128 | — | — | — | AES-128 Key Wrap |
| 1 | AES-GCM-128 | ECDSA P-256 | ECDH P-256 | SHA-256 | AES-128 Key Wrap |
| 2 | AES-GCM-256 | ECDSA P-384 | ECDH P-384 | SHA-384 | AES-256 Key Wrap |

**Наш выбор:** Suite 1/2 (ECDSA/ECDH, forward secrecy) — см. [ADR-0007](../decisions/ADR-0007-dlms-cosem-adapter.md).
Слоты 3–15 зарезервированы (post-quantum криптография).
## Управление ключами

- **Static keys** (Suite 0): загружаются при производстве / через PKI (X.509).
- **Agreed keys** (Suite 1/2): ECDH-обмен при association, ключи сессии не передаются по каналу.
- **Key update**: AES Key Wrap (RFC 3394) для ротации dedicated keys без разрыва связи.
- **Broadcast key**: массовое обновление ключей на флоте счётчиков.

## Практические рекомендации

- **Минимум для биллинга:** HLS + Suite 0 (AES-GCM-128) с ротацией ключей не реже 1 раза в год.
- **Рекомендуемый уровень:** HLS-ECDSA (ID 7) + Suite 1/2 — взаимная аутентификация по сертификатам, non-repudiation, forward secrecy.
- **Invocation counter:** всегда проверять (защита от replay); хранить в нестираемой памяти.
- **LLS** — только commissioning, не в операционном режиме.
- **Транспортный канал:** поверх TCP/IP дополнительно TLS 1.2/1.3 (DLMS-шифрование защищает APDU, но не TCP-канал).
- **RBAC:** привязывать права к логическим группам (no-security / low-security / high-security) через Association Object.

## ProtocolAdapter (контракт)

DLMS/COSEM — **`ProtocolAdapter`** в `src/protocols/dlms.rs` (TBD, P2/P3). Контракт — как у `modbus`, `matter`, `mqtt`:

~~~rust
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn priority(&self) -> Priority;
    fn heartbeat_name(&self) -> Option<&'static str>;
    async fn start(&self);
    fn health_check(&self, registry: &Registry, timeout_ms: u64) -> bool;
    async fn send(&self, msg: BusMessage) -> Result<(), String>;
    async fn recv(&self) -> Option<BusMessage>;
}
~~~

## Связь с проектом

- **PLATE-A (Hard)** — DLMS поверх HDLC (RS-485) или TCP/IP (WAN) — **`Wild` → `Enriched`**;
- **PLATE-C (Life)** — DLMS-шлюз (HES/MDMS) — **`Explored`**;
- **Фронтир** — см. [ADR-0008](../decisions/ADR-0008-frontier-term.md);
- **DCU** — см. [dcu.md](dcu.md).

## Ссылки

- [ADR-0007: DLMS/COSEM adapter](../decisions/ADR-0007-dlms-cosem-adapter.md) — решение, HLS-ECDSA, Suite 1/2, X.509;
- [ADR-0008: Фронтир](../decisions/ADR-0008-frontier-term.md) — 7 состояний;
- [dcu.md](dcu.md) — DCU как частный случай фронтира;
- [glossary](../glossary.md#протоколы) — термины `DLMS/COSEM`, `HLS`, `Security Suite`;
- [TODO.md](../../TODO.md#p2) — P2 `src/protocols/dlms.rs`.

