# Безопасность IoT (TLS 1.3 / DTLS / mTLS / PSK / Secure Boot / OSCORE)

**Статус:** accepted
**Связанные:** [dlms-cosem.md](dlms-cosem.md), [north-south.md](north-south.md), [ADR-0007](../decisions/ADR-0007-dlms-cosem-adapter.md), [ADR-0011](../decisions/ADR-0011-tablet-first-telemetry.md), [glossary](../glossary.md).

## Транспортное шифрование

| Сценарий | Протокол | Шифрование |
|---|---|---|
| IP-устройства (Wi-Fi, LTE) | **MQTT + TLS 1.3** (порт 8883) | AES-128-GCM / AES-128-CCM |
| Constrained (UDP, LoRaWAN, NB-IoT) | **CoAP + DTLS 1.2/1.3** (порт 5684) | AES-128-CCM-8 (RFC 7925) |
| Промышленный | **OPC UA** (встроенная security) | AES-256-GCM + RSA/ECC |
| Данные на устройстве | **AES-256** | — |

## Аутентификация устройств

- **mTLS (Mutual TLS)** — устройство и сервер обмениваются X.509 сертификатами. Частный ключ в Secure Element / TPM.
- **PSK (Pre-Shared Key)** — для constrained-устройств (LoRaWAN, CoAP). Ключ ≥ 128 бит.
- **Raw Public Key** — альтернатива сертификатам для устройств с малой памятью (ECC P-256).
- **PKI-инфраструктура** — централизованное управление сертификатами, автоматическое обновление (renewal при ⅔ срока жизни).

## Архитектурные меры

| Уровень | Мера |
|---|---|
| Hardware root of trust | TPM / Secure Element для хранения ключей |
| Secure Boot | Boot ROM → Bootloader (ECDSA P-256) → Firmware (rollback protection) |
| OTA-обновления | Подписанный firmware, защита от rollback |
| Сегментация сети | Изоляция OT (полевые устройства) от IT |
| Key rotation | Регулярная ротация PSK / обновление сертификатов |
| Forward Secrecy | (EC)DHE + PSK для защиты от записи |

## Специфика constrained-устройств

- **DTLS вместо TLS** (нет TCP handshake, работает поверх UDP, обрабатывает потерю пакетов).
- **Session resumption / 0-RTT** — сокращение handshake для экономии энергии.
- **OSCORE** (Object Security for Constrained RESTful Environments) — end-to-end шифрование CoAP-сообщений, не зависящее от транспорта.
- **LoRaWAN** — встроенное AES-128 шифрование (NwkSKey + AppSKey), но **hop-by-hop**, не end-to-end.

## В проекте

- **mTLS** — планшет (`certs/`, `docs/api/telemetry.md`, [ADR-0011](../decisions/ADR-0011-tablet-first-telemetry.md));
- **DLMS HLS-ECDSA (7) + Suite 1/2** — [dlms-cosem.md](dlms-cosem.md), [ADR-0007](../decisions/ADR-0007-dlms-cosem-adapter.md);
- **X.509** — `certs/` (mTLS, DLMS);
- **Invocation counter** — нестираемая память (DLMS, replay);
- **TLS 1.2/1.3** — поверх TCP/IP (MQTT, HTTP).

## Ссылки

- [dlms-cosem.md](dlms-cosem.md) — DLMS/COSEM (HLS-ECDSA, Suite 1/2, X.509);
- [north-south.md](north-south.md) — протоколы;
- [ADR-0007](../decisions/ADR-0007-dlms-cosem-adapter.md) — DLMS adapter;
- [ADR-0011](../decisions/ADR-0011-tablet-first-telemetry.md) — планшет (mTLS);
- [glossary](../glossary.md) — термины.
