# PLATE-B (Soft) — edge-шлюз (NB-IoT + LTE-M + LoRaWAN + Thread/Zigbee)

**Статус:** спецификация (ADR-0004, обновление).
**Связанные:** [ADR-0004](../decisions/ADR-0004-router-gateway-domains.md), [router-telemetry.md](../protocols/router-telemetry.md), [glossary](../glossary.md).

## Роль

PLATE-B (Soft) — **edge-шлюз** для **ЖКХ** (кондиционирование, тепло, ГВС/ХВС, вентиляция, газ, энергия, освещение), **сигнальной/охранной** службы, **критической** инфраструктуры (РЖК, ГОК), **полевых** мобильных объектов.

**Отдельное устройство** вне корпуса концентратора (x86).

## Платформа (x86 edge-шлюз)

| Параметр | Значение |
|---|---|
| CPU | AMD x86, **16+ ядер** |
| RAM | **128 ГБ** |
| Storage | **SSD RAID** |
| Network | **10GbE** (мин. 1GbE) |
| Security | **TPM 2.0** + Secure Boot |
| Wi-Fi | **нет** (для сервера — только проводной Ethernet) |

## NB-IoT (основной LPWAN)

| Параметр | Значение |
|---|---|
| Стандарт | 3GPP Rel-13 (2016), LTE Advanced Pro |
| Полоса | **180–200 кГц** |
| Энергоэффективность | PSM/eDRX, **10+ лет** от батареи |
| Покрытие | **164 дБ MCL** (подвалы, здания, сельская местность) |
| Скорость | **250 кбит/с**, задержка **1,6–10 с** |
| Плотность | **50 000** устройств/БС |
| Спектр | Лицензированный (оператор), in-band/guard-band/standalone |
| Применение | Счётчики (вода, газ, энергия), мониторинг, освещение, логистика, с/х |

### NB-IoT в России (ГКРЧ 28.12.2017, протокол № 17-44)

| LTE Band | UL (МГц) | DL (МГц) |
|---|---|---|
| B31 (450) | 453–457,4 | 463–467,4 |
| B20 (800) | 832–862 | 791–820 |
| B8 (900) | 880–915 | 925–960 |
| B3 (1800) | 1710–1785 | 1805–1880 |
| B1 (2100) | 1920–1980 | 2110–2170 |
| B7 (2600) | 2500–2570 | 2620–2690 |

**Операторы:** МТС (B3, B20), МегаФон (B20, B8, B3), Билайн (B3).

**Новые IoT-диапазоны (март 2021):** 301–305,8 МГц, 337–341,8 МГц, 343–344 МГц, 390–394 МГц; федеральная IoT-сеть **450 МГц**.

## LTE-M (Cat-M1) — опционально

| Параметр | NB-IoT | LTE-M |
|---|---|---|
| Полоса | 180 кГц | 1,4 МГц |
| Скорость | ~127/159 кбит/с | ~1 Мбит/с |
| Задержка | 1,6–10 с | 50–100 мс |
| Мобильность | нет (reselect) | до 100 км/ч |
| VoLTE | нет | да |
| MCL | 164 дБ | ~156 дБ |
| Батарея | 10+ лет | 5–10 лет |
| Модуль | ~$4–6 | ~$5–15 |

**Выбор:** NB-IoT — статичные (счётчики, датчики, парковки, освещение); LTE-M — движущиеся (трекеры, носимые, логистика, видео, OTA).

## Полевые/edge-узлы

| Роль | Протокол | Плата |
|---|---|---|
| Mesh-шлюз (мост в IP) | Thread/Zigbee | **ESP32-C6** |
| Низкопотребляющие mesh-узлы | Thread/Zigbee | **ESP32-H2** |
| Дальний LPWAN | LoRaWAN | **ESP32-S3/C3 + LoRa-трансивер** |
| Сотовый IoT | LTE Cat-M1 / NB-IoT | **плата с модемом Cat-M1/NB-IoT (SIM)** |

### NB-IoT через USB-свисток

- **SIM7080G** — Cat-M1 + NB-IoT, глобальные диапазоны (B1/B2/B3/B4/B5/B8/B12/B13/B14/B18/B19/B20/B25/B26/B27/B28/B66/B71/B85), UART 115200 8N1, AT-команды, Nano SIM, TCP/UDP/HTTP/HTTPS/TLS/DTLS/PING/LWM2M/CoAP/MQTT, PSM/eDRX;
- **SIM8262E-M2** — 5G/4G, M.2, для x86 (USB-адаптер);
- **M5Stack Unit CatM (SIM7080G)** — пример: https://shop.m5stack.com/products/sim7080g-cat-m-nb-iot-unit

## LoRaWAN (внешний шлюз)

- **zhongyismart** `ZY2S208` — IP67, PoE, 2G/3G/4G, 8 каналов: http://zhongyismart.cn/en/products/lo-ra-wan-outdoor-gateway
- Свой **ESP32-S3/C3 + LoRa** (SX1262/SX1276) — альтернатива.

## Протоколы (сервер/edge-шлюз)

- **MQTT** — основной транспорт шлюз→сервер (TCP 1883 / TLS 8883); брокер (Mosquitto/EMQX/HiveMQ);
- **CoAP** — шлюз ↔ ограниченные устройства (UDP, малый оверхед);
- **LoRaWAN / NB-IoT** — распределённые датчики;
- **Modbus RTU/TCP**, **OPC-UA**, **RS-485**, **BACnet** — промышленная автоматика;
- **DLMS/COSEM** — AMI (см. [dlms-cosem.md](../protocols/dlms-cosem.md)).

## Ссылки

- [ADR-0004: Router/gateway domains](../decisions/ADR-0004-router-gateway-domains.md) — PLATE-B = NB-IoT + edge-шлюз;
- [router-telemetry.md](../protocols/router-telemetry.md) — SNMP/NetFlow/Syslog;
- [dcu.md](../protocols/dcu.md) — DCU как частный случай фронтира;
- [glossary](../glossary.md) — термины `NB-IoT`, `LTE-M`, `LoRaWAN`, `MQTT`, `CoAP`;
- [TODO.md](../../TODO.md) — P2.26 (NB-IoT), P2.27 (LTE-M).

