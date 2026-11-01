# Raw — абстракция (P3.37)

**Статус:** спецификация (P3.37).
**Связанные:** [plate-c-life.md](plate-c-life.md), [glossary.md](../glossary.md).

## Идея

Raw — не один тип, а интерфейс. Слои:
1. Сырой поток (RawSource): сетевая карта (pcap, DPDK, AF_XDP), файл, Redis, CBOR.
2. Структурирование (StreamParser): байты -> поля (протоколы, timestamps).
3. Статистика: агрегаты (частота, распределения, аномалии).
4. Паттерны: корреляции, триггеры, аномалии.

## Трейты

- trait RawSource { fn read(&mut self) -> Result<Bytes>; }
- trait StreamParser { fn parse(&self, bytes: &[u8]) -> Result<Structured>; }
- trait Statistics { fn aggregate(&self, data: &[Structured]) -> Stats; }
- trait Patterns { fn detect(&self, stats: &Stats) -> Vec<Pattern>; }

## Что сейчас (факты)

- storage/raw.rs — RawStore (CBOR, 10 МБ, 24 ч) — лог, не «сырой поток».
- storage/cache.rs — Redis (sensor:last), flush_to_raw.
- migrations/003_telemetry.sql — TimescaleDB (telemetry).

## Что делать (P3.37)

- trait RawSource (pcap/DPDK/AF_XDP) — позже.
- trait StreamParser (протоколы) — позже.
- Статистика/паттерны — позже.
- Сейчас: P3.34 (telemetry::history/stream), P3.36 (реструктуризация protocols/).

## Диагностика по каждому интерфейсу

- RawSource — на каждый интерфейс (eth0, eth1, Wi-Fi, DPDK-порт).
- alive.<iface> — по каждому, health_check — по каждому.
- Отказ eth0 не роняет eth1 (локальные отказы).

## Алармы и нестабильность

- Алармы: пороги (потери, джиттер, retransmits).
- Нестабильность: динамика (частота, распределение, аномалии).
- Триггеры: на паттерны (worm, miner, DDoS).

## Реальные данные (не эмуляция)

- RawSource: Pcap, Dpdk, AfXdp, File (pcap-файл, для демо), Cbor (RawStore).
- Демо — File, продакшн — Pcap/Dpdk.

## Безопасность (паттерны)

- Worm, miner, DDoS, нехарактерная активность — паттерны в статистике.
- Не open relay, Unicast P2P (см. router-gateway.md).
