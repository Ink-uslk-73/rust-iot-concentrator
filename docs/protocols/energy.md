# Энергоэффективность (LPWAN, BLE, CoAP vs MQTT vs HTTP)

**Статус:** accepted
**Связанные:** [north-south.md](north-south.md), [plate-b-soft.md](../system/plate-b-soft.md), [glossary](../glossary.md).

## LPWAN / BLE

| Протокол | Потребление (TX) | Батарея | Дальность | Данные |
|---|---|---|---|---|
| **LoRaWAN** | ~100 µAh/сообщение (44 B) | до 15 лет | 5–15 км | 0.3–50 kbps |
| **Sigfox** | ~50–980 mWs/сессия | ~5 лет | 10–50 км | ~100 B/сообщение |
| **NB-IoT** | ~1.8 mAh/сообщение (44 B) | до 10 лет | >10 км (оператор) | до 250 kbps |
| **LTE-M** | ~217 µW средняя | ~6.8 лет | >10 км | до 1 Mbps |
| **BLE** | 10–100 mW (duty cycle) | месяцы–годы | 10–100 м | 1–2 Mbps |
| **Zigbee** | ~5–10 mW | 1–5 лет (mesh) | 10–100 м | 250 kbps |

## Ключевые выводы

- **LoRaWAN** — самый энергоэффективный LPWAN: **~10× меньше энергии** на сообщение, чем **NB-IoT**. Оптимально для **периодической** телеметрии (раз в минуту/час).
- **NB-IoT** — эффективнее при **крупных пакетах** (1280 B → 61 nJ/bit vs 480 nJ/bit для 80 B), но **дороже** по энергии на **join** (3 mAh vs 1 mAh у LoRaWAN).
- **BLE** — лучший для **короткой дистанции** с умеренным объёмом данных (носимые, бейджи).
- **CoAP** (4 B header) энергоэффективнее **MQTT** (2 B header, но TCP overhead) и **HTTP** (~700 B header) на уровне приложения.

## Практические рекомендации

- **Duty cycling** + адаптивная скорость передачи → экономия 25–33% энергии.
- **LoRaWAN**: Class A (только uplink) — минимальное потребление; Class C — для low-latency.
- **NB-IoT**: PSM (Power Saving Mode) + eDRX для глубокого сна.

## В проекте

- **PLATE-B (Soft)**: NB-IoT/LTE-M/LoRaWAN/BLE/Zigbee (см. [plate-b-soft.md](../system/plate-b-soft.md));
- **NB-IoT**: PSM/eDRX (10+ лет) — [plate-b-soft.md](../system/plate-b-soft.md);
- **LoRaWAN**: Class A — [north-south.md](north-south.md);
- **CoAP**: для constrained-устройств — [north-south.md](north-south.md).

## Ссылки

- [north-south.md](north-south.md) — протоколы;
- [plate-b-soft.md](../system/plate-b-soft.md) — NB-IoT/LTE-M/LoRaWAN;
- [glossary](../glossary.md) — термины;
- [TODO.md](../../TODO.md) — P2.26 (NB-IoT), P2.27 (LTE-M).
