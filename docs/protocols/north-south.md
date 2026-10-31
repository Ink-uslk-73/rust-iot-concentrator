# Протоколы: север (облако) / юг (устройства)

**Статус:** accepted
**Связанные:** [ADR-0016](../decisions/ADR-0016-concentrator-roles.md), [conversion.md](conversion.md), [dcu.md](dcu.md), [dlms-cosem.md](dlms-cosem.md), [glossary](../glossary.md).

## Север (к облаку / бэкенду)

| Протокол | Транспорт | Модель | Применение |
|---|---|---|---|
| **MQTT** (v3.1.1/5.0) | TCP | Pub/Sub (broker) | Телеметрия, управление (~63% IoT) |
| **CoAP** (RFC 7252) | UDP | Req/Resp + Observe | Ограниченные, NB-IoT, LwM2M |
| **HTTP(S)** | TCP | Req/Resp | Web-интеграция, API, дашборды |
| **AMQP** | TCP | Очереди | Enterprise, надёжная доставка |
| **LwM2M** (OMA) | UDP (CoAP) | Управление жизненным циклом | NB-IoT/LTE-M модули |
| **OPC UA** | TCP/UDP | Client/Server + Pub/Sub | Промышленный IoT, SCADA |

## Юг (к устройствам)

| Протокол | Тип | Дальность | Использование |
|---|---|---|---|
| **LoRaWAN** | LPWAN (sub-GHz) | 2–15 км | Частные сети, smart metering |
| **NB-IoT** | Сотовый (LTE) | >10 км | Счётчики, asset tracking |
| **LTE-M** | Сотовый | >10 км | Трекинг, логистика |
| **Zigbee / Z-Wave** | Mesh (802.15.4) | 10–100 м | Умный дом |
| **BLE** | P2P | ~50 м | Носимые, бейджи |
| **Wi-Fi** | IP | ~50 м | Камеры, шлюзы |
| **M-Bus / WM-Bus** | P2P | 10–300 м | Умные счётчики (вода, газ, тепло) |
| **PLC (G3-PLC, PRIME)** | По ЛЭП | 1–3 км | Smart grid, AMI |
| **Modbus RTU/TCP** | RS-485 / IP | — | Промышленные PLC, SCADA |
| **DNP3 / IEC 60870-5-104** | IP / RS-485 | — | Энергетика, телеметрия |

## Типовая архитектура

```
[Датчики] ──LoRaWAN/Zigbee/M-Bus──▶ [Концентратор/Шлюз] ──MQTT/CoAP──▶ [Облако / MDM]
```

## Примеры

- **LoRaWAN-шлюз** — кадры LoRaWAN → MQTT (broker);
- **PLC-концентратор** (G3-PLC) — счётчики → MQTT/HTTP в MDM;
- **Промышленный** (RS-485) — Modbus → OPC UA/MQTT для SCADA.

## В проекте

- **Север**: MQTT (PLATE-C), HTTP/REST (API), OPC UA (P4.1);
- **Юг**: Modbus (PLATE-A), DLMS (PLATE-A/C), NB-IoT/LTE-M/LoRaWAN (PLATE-B), Thread/Zigbee (PLATE-B);
- **Управление**: LwM2M (P4.11), SNMP (Router WI, ADR-0010).

## Ссылки

- [conversion.md](conversion.md) — конвертация (юг → север);
- [dcu.md](dcu.md) — DCU (NAN/WAN);
- [dlms-cosem.md](dlms-cosem.md) — DLMS/COSEM;
- [router-telemetry.md](router-telemetry.md) — SNMP/NetFlow;
- [glossary](../glossary.md) — термины;
- [ADR-0016](../decisions/ADR-0016-concentrator-roles.md) — роли концентратора.
