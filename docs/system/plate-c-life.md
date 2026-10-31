# PLATE-C (LIFE) — скоростные каналы, DAS/JBOD, облака

**Статус:** спецификация.
**Связанные:** [ADR-0006](../decisions/ADR-0006-satellite-sovereignty.md), [ADR-0018](../decisions/ADR-0018-wrx90e-platform.md), [pcie-topology.md](pcie-topology.md).

## Роль

PLATE-C (LIFE, ~1000 мс) — скоростные каналы (оптика, WiFi), DAS/JBOD, бэки,
облака/озёра данных, Gateway, маршрутизатор.

**Отдельный компьютер** (4U) в стойке, WRX90E. Автономная перезагрузка, смена ПО, диагностика.

## Платформа

| Параметр | Значение |
|---|---|
| CPU | AMD Threadripper PRO (sTR5, WRX90E) |
| Память | 8× DDR5 R-DIMM (до 2TB) |
| PCIe | 7× PCIe 5.0 x16 (6× x16 + 1× x8) |
| BMC | AST2600 + 1G LAN (автономная диагностика) |
| Сеть | 2× 10G (Intel X710-AT2) |
| Питание | свой БП → PCIe (резерв) → ИБП |

## Каналы (LIFE, 1000 мс)

- Оптика, WiFi (скоростные каналы)
- DAS/JBOD (внешние массивы)
- Бэки, облака/озёра данных
- Gateway, маршрутизатор

## Данные

- **alive.PLATE-C** — «компьютер жив», раз в 1 с.
- **store & forward**: Raw локально (CBOR, 10 МБ, 24 ч), досылка в CORE по PCIe P2P.
- **P2P**: Alveo ×2 на CPU1 (FPGA-to-FPGA), Alveo ↔ NVMe.

## Открытые вопросы

- [ ] Оптика: 10G/25G/100G (какая модель)?
- [ ] WiFi: какой модуль (WiFi 6/6E/7)?
- [ ] DAS/JBOD: сколько дисков, какой интерфейс (SAS/SATA/NVMe)?
- [ ] Бэки/облака: протокол (S3, NFS, CIFS)?
## Wi-Fi (M.2 Key E)

- WRX90E-SAGE SE **не имеет встроенного Wi-Fi**, но есть слот **M.2 Key E (PCIe)**.
- Модуль Wi-Fi 6/6E (например, ASUS WiFi-AX5000) — **опционально**, для:
  - локальной меш-сети (смартфоны/планшеты/датчики);
  - резервного выхода в облака (если оптика недоступна).
- Без модуля — PLATE-C только с 2× 10G (проводной).

## Real (transit) роутер

- PLATE-C может быть **transit-роутером**: пропускает трафик, не терминирует.
- **Не open relay**: аутентификация обязательна (mTLS, WireGuard/IPsec).
- Все соединения к «своим» (CORE, сателлиты, облако) — через **шифрованные туннели**.
## MAN (Metropolitan Area Network)

- **MAN** — замыкание трафика внутри территории (ТОСЭР, ГОК, сортировочный): кабель + Wi-Fi.
- **Не выход в интернет**: трафик между концентраторами/сателлитами, внутри.
- **Режим**: только **Unicast P2P** (по одному); multicast/broadcast/anycast — **не использовать**.
## Непрерывная синхронизация (MAN)

- **MAN**: локальная территория, трафика хватает на всех (10–12 концентраторов + сателлиты).
- **Непрерывно**: потоки (Raw → CORE, Redis, TimescaleDB) синхронизируются постоянно.
- **Unicast P2P**: по одному; multicast/broadcast/anycast — не использовать.
## Транзакции и версии (распределённые СУБД)

- **Redis (hot-path)** — CP: strong consistency, MULTI/EXEC (не 2PC).
- **TimescaleDB (агрегаты)** — CP: ACID, 2PC (prepared transactions).
- **Raw CBOR (store & forward)** — AP: eventual consistency, версия по timestamp/offset, не транзакции.
- **Satellite-Raw** — AP: локально, досылка при оживлении, версия по timestamp.
- **«Транзакция, иначе версия разойдётся»** — для Redis/TimescaleDB; Raw — eventual, версия по timestamp/offset.