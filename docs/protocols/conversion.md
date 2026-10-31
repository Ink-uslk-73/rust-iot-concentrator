# Конвертация: юг → север (трёхуровневая трансформация)

**Статус:** accepted
**Связанные:** [north-south.md](north-south.md), [dcu.md](dcu.md), [dlms-cosem.md](dlms-cosem.md), [glossary](../glossary.md).

## Трёхуровневая трансформация

```
Южный протокол (сырые данные)
    │
    ▼
┌─────────────────────────────────────────┐
│  1. Драйвер/декодер южного протокола     │  ← парсинг бинарных кадров
│  2. Нормализация (tag mapping, scaling) │  ← типизированные значения
│  3. Буферизация (store-and-forward)     │  ← локальный буфер при обрыве
│  4. Формирование северного payload       │  ← JSON / Sparkplug B / OPC UA
│  5. Публикация по северному протоколу    │  ← MQTT / CoAP / OPC UA
└─────────────────────────────────────────┘
    │
    ▼
Северный протокол (MQTT, HTTP, OPC UA)
```

## Типовые пары

| Южный (вход) | Северный (выход) | Что делает шлюз |
|---|---|---|
| Modbus RTU/TCP | MQTT (JSON) | Polling регистров → tag → JSON → publish |
| CoAP (устройства) | MQTT (облако) | CoAP Observe → MQTT pub/sub (Eclipse Ponte, EMQ CoAP) |
| LoRaWAN (Raw) | MQTT / HTTP | Decode LoRa frame → payload → forward к Network Server |
| OPC UA (PLC) | MQTT / Sparkplug B | UA read → metric mapping → publish к UNS |
| Zigbee / Z-Wave | MQTT | ZCL/Z-Wave frames → JSON → MQTT |

## Ключевые принципы

1. **Не публикуйте сырые байты** — парсите и стандартизируйте (Sparkplug B, JSON с `timestamp`, `device_id`, `quality`).
2. **Store-and-forward** — при обрыве сети данные буферизуются локально и доставляются при восстановлении (MQTT QoS 1/2).
3. **Гибридные паттерны** — CoAP на устройствах → MQTT на шлюзе → HTTP для облачного API.

## В проекте

- **`src/protocols/`** — `ProtocolAdapter` (modbus, matter, mqtt) + `dlms`, `router_telemetry`, `nb_iot`, `lte_m` (P2.22, P2.24, P2.26, P2.27);
- **`src/satellite/peer_agent.rs`** — `PeerLink` + `StoreForward` + `PeerAgent` (store-and-forward);
- **`src/core/frontier.rs`** — `FrontierState` (7) — аналитика (Raw → Patterned).

## Ссылки

- [north-south.md](north-south.md) — протоколы;
- [dcu.md](dcu.md) — DCU (NAN/WAN);
- [dlms-cosem.md](dlms-cosem.md) — DLMS/COSEM;
- [glossary](../glossary.md) — термины;
- [TODO.md](../../TODO.md) — P2.22, P2.24, P2.26, P2.27.
