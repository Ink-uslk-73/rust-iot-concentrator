# DCU (Data Concentrator Unit) и IoT-концентратор

**Статус:** контекст, не требование (наш концентратор — **шире** DCU).
**Связанные:** [ADR-0008](../decisions/ADR-0008-frontier-term.md), [dlms-cosem.md](dlms-cosem.md), [glossary](../glossary.md).

## DCU (Data Concentrator Unit)

DCU — устройство, ядро **AMI** (Advanced Metering Infrastructure) и **Smart Grid**:
агрегирует данные со множества умных счётчиков и передаёт их на серверы
коммунальных предприятий (HES/MDMS).

**Функциональность:**
- маршрутизатор и шлюз: сбор показаний (потребление, напряжение) через **NAN**;
- предобработка, фильтрация, сжатие данных;
- передача в HES/MDMS через **WAN**;
- безопасность: шифрование (AES-GCM), аутентификация (X.509).

**Применение:** энергоменеджмент, утечки воды, качество воздуха, микросети.

**Альтернативы:** PDC (Phasor Data Concentrator), DCU в авиации.

## NAN / Downstream (DCU ↔ счётчики)

| Технология | Стандарт | Частота / Среда |
|---|---|---|
| PLC (G3-PLC, PRIME, IEEE 1901.2) | OFDM | 10–490 кГц (G3-PLC), 2–6 МГц (BPLC) |
| RF mesh (IEEE 802.15.4g, ZigBee) | Sub-1 GHz | 865–867 МГц, 915 МГц (ISM) |
| LoRaWAN | LoRa | Sub-1 GHz |
| RS-485 | — | 1200–115200 бод |
| Wireless M-Bus (EN 13757-4) | — | Sub-1 GHz |

## WAN / Uplink (DCU ↔ HES/MDMS)

| Технология | Примечание |
|---|---|
| 4G LTE (Cat 4/1) | Основной backhaul; fallback 2G/3G, dual-SIM |
| GPRS / 3G | Альтернатива в регионах без LTE |
| Ethernet 10/100 Мбит/с | RJ-45, оптоволокно |
| Fiber optic | Прямое подключение к HES |


## IoT-концентратор (шире DCU)

**IoT-концентратор** — устройство или программный уровень (**edge gateway**),
который агрегирует данные от множества датчиков и IoT-устройств, обеспечивая
их связь с облачными платформами или центральными серверами.

**Функции:** сбор данных, предобработка, конвертация протоколов (Zigbee, Z-Wave,
LoRaWAN → IP), удалённое управление, безопасность (шифрование).

**Архитектура:** уровень **пограничного шлюза** (fog), связующее звено между
полевыми устройствами и облаком (Azure IoT Hub, AWS IoT).

**Примеры:** MG-E3 (PLC-IoT), Samsung SmartThings / Aeotec Smart Home Hub
(Matter/Thread/Zigbee), «Гермес» (LoRaWAN, M-Bus, GSM/Ethernet), Azure IoT Hub.

**Отличие от старых hub:** старые hub дублируют трафик на все порты; современные
IoT-концентраторы — интеллектуальные устройства (ARM + Linux) с edge computing.

**Терминология не установилась** — отсюда новации (`Фронтир`, `Dew Computing`).

## Фронтир

**Фронтир** (Frontier) — граница между «освоенной территорией» (облако, HES/MDMS)
и «дикой землёй данных» (сенсоры, счётчики, IoT-устройства). **Подвижная** —
расширяется по мере освоения данных. **7 состояний** (`Wild`…`Explored`),
аналогия — «улитка Улама» (Ulam spiral). См. [ADR-0008](../decisions/ADR-0008-frontier-term.md).

**Наш концентратор** — **на фронтире**: агрегирует, предобрабатывает,
конвертирует протоколы, обеспечивает безопасность.

## Application Layer

- **DLMS/COSEM** (IEC 62056-5-3 / 6-1 / 6-2) — основной протокол AMI, см. [dlms-cosem.md](dlms-cosem.md);
- **IEC 60870-5-104** — телеметрия/управление в энергосистемах;
- **Modbus RTU / Modbus TCP** — промышленные счётчики и субстанции;
- **MQTT** — лёгкий протокол для облачных платформ;
- **IEC 61850** — DCU в вторичных подстанциях (Smart Grid).

## Security

- **TLS 1.2/1.3** — шифрование WAN-канала;
- **AES-GCM-128** — шифрование данных DLMS/COSEM;
- **X.509** — аутентификация устройств (см. [ADR-0007](../decisions/ADR-0007-dlms-cosem-adapter.md));
- **TACACS+ / LDAPS** — управление доступом к DCU.

## Наш концентратор

**Интегрированное устройство**, **не только DCU**: телеком + IoT + AMI + SCADA,
**на фронтире**. Поддерживает несколько физических сред (PLC + RF + Ethernet)
и несколько прикладных протоколов (DLMS/COSEM + Modbus + MQTT), что позволяет
гибко адаптироваться под конкретную инфраструктуру.

**PLATE-A (Hard)** = AIG-502 — fog-узел, Modbus/DLMS;
**PLATE-B (Soft)** — TODO;
**PLATE-C (Life)** — несущая + ASUS + FPGA SmartNIC, СУБД, облака, шлюз.

## Ссылки

- [ADR-0008: Фронтир](../decisions/ADR-0008-frontier-term.md) — 7 состояний, улитка Улама;
- [ADR-0007: DLMS/COSEM adapter](../decisions/ADR-0007-dlms-cosem-adapter.md) — HLS-ECDSA, Suite 1/2, X.509;
- [dlms-cosem.md](dlms-cosem.md) — полный DLMS/COSEM;
- [glossary](../glossary.md#архитектура) — термины `AMI`, `NAN`, `WAN`, `HES/MDMS`;
- [TODO.md](../../TODO.md) — P2 `src/protocols/dlms.rs`, P4 `DLMS on demand`.


