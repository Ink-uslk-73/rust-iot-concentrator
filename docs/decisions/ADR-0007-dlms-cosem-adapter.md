# ADR-0007: DLMS/COSEM adapter (HLS-ECDSA, Suite 1/2, X.509)

**Статус:** принято (P2/P3, надстройка)
**Дата:** 2026-10-30

## Контекст

DLMS/COSEM (IEC 62056-5-3 / 6-1 / 6-2) — основной протокол обмена со счётчиками
в AMI/Smart Grid. Требование к концентратору: поддерживать DLMS как
`ProtocolAdapter`, но **фоном** (надстройка, не блокирует основную работу).

## Решение

- **`ProtocolAdapter`** в `src/protocols/dlms.rs` (будущий), контракт — как у
  `modbus`, `matter`, `mqtt`: `send/recv`, `health_check(&Registry, timeout_ms)`,
  `heartbeat_name() -> Option<&'static str>`.
- **Access Security:** **HLS-ECDSA (ID 7)** — взаимная аутентификация client ↔ server
  по сертификатам, non-repudiation. LLS (ID 1) — **только commissioning**, не в
  операционном режиме. No Security (0) — только Public Client (адрес 16).
- **Transport Security:** **Suite 1/2** (AES-GCM-128/256), ECDH для forward secrecy,
  AES Key Wrap (RFC 3394) для ротации dedicated keys.
- **X.509** — сертификаты для аутентификации (уже есть `certs/`, mTLS в `docs/api/`).
- **Invocation counter** — нестираемая память, защита от replay.
- **TLS 1.2/1.3** поверх TCP/IP — DLMS-шифрование защищает APDU, но не TCP-канал.

## Обоснование

- **Безопасность** — биллинг, non-repudiation, forward secrecy;
- **Совместимость** с AMI/Smart Grid (DLMS — де-факто стандарт);
- **Надстройка** — не блокирует основную работу (modbus/matter/mqtt);
- **X.509** — единая PKI с mTLS (сателлиты ↔ концентратор).

## Что НЕ входит

- **Реализация** `src/protocols/dlms.rs` — **P2/P3**, не сейчас;
- **`Cargo.toml`** (`dlms`, `p256`, `p384`, `x509-parser`) — **позже**, когда начнём;
- **LLS** — только commissioning, не production.

## Последствия

- `docs/protocols/dlms-cosem.md` — полный DLMS/COSEM (Access/Transport/Suites/Key management);
- `docs/protocols/dcu.md` — DCU как частный случай фронтира (ссылка на ADR-0008);
- `docs/glossary.md` — `## Протоколы` (`DLMS/COSEM`, `HLS`, `Security Suite`, ...);
- `TODO.md` — `P2` `src/protocols/dlms.rs` (надстройка, фоном);
- `src/protocols/dlms.rs` — создать (TBD, P2/P3).

## Ссылки

- [DLMS/COSEM](../protocols/dlms-cosem.md) — полный протокол;
- [DCU](../protocols/dcu.md) — DCU как частный случай фронтира;
- [Frontier term](ADR-0008-frontier-term.md) — фронтир (7 состояний);
- [Satellite sovereignty](ADR-0006-satellite-sovereignty.md) — митрополия/протектораты;
- [docs/glossary.md](../glossary.md) — термины `DLMS/COSEM`, `HLS`, `Security Suite`;
- [TODO.md](../../TODO.md) — P2 `src/protocols/dlms.rs`, P4 `DLMS on demand`.

