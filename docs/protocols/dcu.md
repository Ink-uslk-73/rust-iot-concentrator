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

