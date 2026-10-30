# ADR-0015: Комбинация как новация (DLMS + NB-IoT + edge-шлюз + Frontier)

**Статус:** принято
**Дата:** 2026-10-30

## Контекст

DLMS/COSEM, NB-IoT, LTE-M, LoRaWAN — **стандарты** (не новации). Но их **комбинация** с **Frontier** (7 состояний) и **edge-шлюзом** (PLATE-B) — **новация**, которая **не** была **зафиксирована** в ADR.

## Решение

Зафиксировать **комбинацию** как **новацию**:

| Компонент | Роль |
|---|---|
| **DLMS/COSEM** | AMI (счётчики, биллинг), HLS-ECDSA + Suite 1/2, X.509 |
| **NB-IoT** | LPWAN (статические счётчики/датчики), лицензированный спектр, 164 дБ MCL |
| **LTE-M** | LPWAN (движущиеся устройства), мобильность, VoLTE |
| **edge-шлюз (PLATE-B)** | x86, 10GbE, TPM 2.0, NB-IoT + LTE-M + LoRaWAN + Thread/Zigbee |
| **Frontier (7 состояний)** | Wild (Dew) → Enriched → ... → Explored (Cloud), «улитка Улама» |
| **Router WI telemetry** | SNMP v3 + NetFlow/IPFIX + Syslog, collector на frontier |

**Что даёт комбинация:**
- **AMI** (DLMS) + **LPWAN** (NB-IoT/LTE-M) + **edge** (PLATE-B) + **Frontier** (7 состояний) — **единая** **архитектура** **от** **счётчика** **до** **облака**;
- **Router WI** — **реальный** **источник** **телеметрии** (**не** **эмуляция**);
- **`FrontierState`** — **код** (**ADR-0008**), **`router_telemetry.rs`** — **P2.24** (TBD).

## Обоснование

- **Стандарты** **не** **новации**, но **их** **комбинация** — **новация**;
- **Frontier** (**7** **состояний**) — **связывает** **Dew/Fog/Edge** **с** **аналитикой**;
- **PLATE-B** (**NB-IoT + edge-шлюз**) — **новая** **роль** (**было** **TODO**);
- **Router WI** — **реальный** **источник** (**не** **эмуляция**).

## Что НЕ входит

- **Реализация** — **P2.22** (DLMS), **P2.24** (Router WI), **P2.26** (NB-IoT), **P2.27** (LTE-M) — **TBD**;
- **CXL / disaggregated** — **ADR-0013** (**TBD**), **P4.12** — **future**.

## Последствия

- **Новация** **зафиксирована** (**ADR-0015**);
- **Комбинация** — **основа** **для** **`ADR-0004`** (**PLATE-B** — **NB-IoT + edge-шлюз**), **`ADR-0010`** (**Router WI**);
- **`glossary.md`** — **`Frontier`** (**имя** **проекта**), **NB-IoT**, **LTE-M**, **DLMS** (**есть**).

## Ссылки

- [ADR-0004: Router/gateway domains](ADR-0004-router-gateway-domains.md) — PLATE-B = NB-IoT + edge-шлюз;
- [ADR-0007: DLMS/COSEM adapter](ADR-0007-dlms-cosem-adapter.md) — HLS-ECDSA, Suite 1/2, X.509;
- [ADR-0008: Фронтир](ADR-0008-frontier-term.md) — 7 состояний, «улитка Улама»;
- [ADR-0010: Router WI telemetry](ADR-0010-router-telemetry.md) — SNMP v3 + NetFlow/IPFIX + Syslog;
- [plate-b-soft.md](../system/plate-b-soft.md) — PLATE-B (NB-IoT + edge-шлюз);
- [dlms-cosem.md](../protocols/dlms-cosem.md) — DLMS/COSEM;
- [router-telemetry.md](../protocols/router-telemetry.md) — SNMP/NetFlow/sFlow/gNMI/Syslog;
- [TODO.md](../../TODO.md) — P2.22 (DLMS), P2.24 (Router WI), P2.26 (NB-IoT), P2.27 (LTE-M).
