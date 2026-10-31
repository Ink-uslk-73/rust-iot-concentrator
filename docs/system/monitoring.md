# Мониторинг IoT-шлюзов (edge / LoRaWAN / cloud)

**Статус:** accepted
**Связанные:** [router-telemetry.md](../protocols/router-telemetry.md), [domain-priority.md](domain-priority.md), [ADR-0010](../decisions/ADR-0010-router-telemetry.md), [glossary](../glossary.md).

## Edge-мониторинг (на самом шлюзе)

| Инструмент | Потребление | Протоколы | Особенности |
|---|---|---|---|
| **Netdata** | <1% CPU, ~30 МБ RAM (ARM) | MQTT, Modbus, 1-Wire, IPMI, SNMP, OTel | Per-second, parent-child, zero-config |
| **Prometheus + Grafana** | node_exporter: ~10 МБ | MQTT/Modbus exporter, OTLP | Экосистема exporters |
| **Zabbix (Agent 2)** | ~50 МБ RAM | MQTT, Modbus (native), SNMP | Авто-обнаружение, алертинг, 10k+ устройств |
| **NATS + TimescaleDB + Grafana** | Легкий (NATS) | MQTT → NATS → DB | Offline, синхронизация с облаком |

**Рекомендация**: для constrained ARM (RPi, Industrial) — **Netdata**; для промышленных Linux — **Prometheus + Grafana** или **Zabbix** (native MQTT/Modbus).

## LoRaWAN-шлюзы

| Платформа | Тип | Что мониторит |
|---|---|---|
| **The Things Stack** | Cloud / self-hosted | Gateway status, uplink/downlink, ADR, RSSI, SNR, GPS |
| **ChirpStack** | Self-hosted (open source) | Gateway health, packet stats, device battery, gRPC API |
| **WisDM Metrics** (RAKwireless) | Cloud (no-code) | CPU, RAM, uptime, SD-card, cellular signal, CRC errors |
| **AWS IoT Core for LoRaWAN** | Cloud | Gateway status, data rate, SF, frame counter, CUPS |

**Ключевые метрики LoRaWAN**: uptime / CPU / RAM / SD-card, cellular signal (RSSI), uplink/downlink, CRC error rate, ADR commands sent.

## Cloud-платформы (fleet management)

| Платформа | Сильные стороны | Ценовая модель |
|---|---|---|
| **Datadog IoT** | IoT Agent (ARM/x64), anomaly detection | ~$15/host/мес |
| **Azure IoT Central** | Device templates, AI-аналитика | ~$0.83/device/мес |
| **AWS IoT Device Management + CloudWatch** | Масштаб, OTA, Device Defender | Usage-based |
| **ThingsBoard** | Self-hosted, rule chains, Modbus/BACnet/OPC-UA gateway | Free / Enterprise |
| **Moxa ThingsPro** | Modbus → MQTT/HTTPS, AWS/Azure SDK, edge analytics | По модели |

## Zabbix vs Prometheus (для IoT-шлюза)

| Критерий | Zabbix | Prometheus |
|---|---|---|
| **MQTT** | `mqtt.get` в Agent 2 (native) | Внешний `mqtt_exporter` |
| **Modbus** | `modbus.get` в Agent 2 (native) | Внешний `modbus_exporter` |
| **SNMP** | Native polling + traps | `snmp_exporter` (нет traps) |
| **Push-модель** | Active checks (за NAT) | Pull (scrape) |
| **Proxy** | Встроенный, offline-буфер | Нет (Pushgateway) |
| **All-in-one** | Agent + Proxy + SNMP + IPMI | Exporters (N штук) |
| **Алертинг** | Встроенный (triggers, escalation) | Alertmanager (отдельный) |
| **UI** | Встроенный (dashboards, maps) | Grafana (отдельный) |
| **Длинное хранение** | SQL, partitioning | TSDB 15 дней + Thanos |

**Для IoT-шлюза в поле** (ARM, RS-485, MQTT, NAT, прерывистая связь) — **Zabbix** (native MQTT/Modbus, push, proxy, all-in-one).
**Для cloud-native / K8s** (high-frequency, OTel) — **Prometheus + Grafana**.

## В проекте

- **`src/metrics/overhead.rs`** — `OverheadMetrics` (hard/api latency, sse jitter, bus buffer, drops, retransmits), `fetch_max` + `swap(0)`;
- **`src/metrics/registry.rs`** — `Registry` (alive.<PLATE>);
- **`src/protocols/router_telemetry.rs`** — SNMP/NetFlow/Syslog (P2.24);
- **`src/api/metrics.rs`** — `/api/v1/metrics`, `/api/v1/metrics/self`;
- **`src/api/telemetry.rs`** — SSE `/api/v1/telemetry/stream` (P2.25);
- **Zabbix** — **не используем** (`Prometheus`-стек **не** **нужен** **сейчас**); **Router WI** — **внешний** **Zabbix** (**опционально**).

## Ссылки

- [router-telemetry.md](../protocols/router-telemetry.md) — SNMP/NetFlow/Syslog;
- [domain-priority.md](domain-priority.md) — HARD/SOFT/LIFE;
- [ADR-0010](../decisions/ADR-0010-router-telemetry.md) — Router WI;
- [glossary](../glossary.md) — термины;
- [TODO.md](../../TODO.md) — P2.24 (Router WI), P2.25 (SSE).
