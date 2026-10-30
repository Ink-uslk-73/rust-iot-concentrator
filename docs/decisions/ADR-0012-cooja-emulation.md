# ADR-0012: Cooja (Contiki-NG) для эмуляции WSN

**Статус:** принято
**Дата:** 2026-10-30

## Контекст

Эмуляция — **основная** (до железа). Для **WSN** (Thread/Zigbee, 6LoWPAN, RPL, 6TiSCH) **не** нужно **изобретать** **велосипед** — **есть** **Cooja** (Contiki-NG).

## Решение

- **Cooja** (Contiki-NG) — симулятор WSN: **кросс-уровневая** симуляция (MSPSim, Sky motes, Cooja motes, Java motes);
- **Радиомодели** — UDGM, DGRM, затухание, интерференция;
- **Режимы** — GUI (отладка), headless (CI/CD);
- **IPv6** — border router (RPL-DAG), tunslip6, `aaaa::x`;
- **RPL** — RPL-Lite (non-storing, 1 DODAG), RPL-Classic (storing, OF0/MRHOF), NULLROUTING;
- **6LoWPAN** (RFC 4944/6282/6775), **6TiSCH** (TSCH);
- **Отладка** — Mote output (printf), MSP Code Watcher (Sky), GDB-stub (Sky, ARM в Cooja-NG), Timeline, pcap (Wireshark).

## Обоснование

- **Готовое** **решение** (не **изобретать** **велосипед**);
- **Проверено** (Contiki-NG, сообщество);
- **CI/CD** (headless: `./gradlew run --args="-nogui=my-simulation.csc"`).

## Что НЕ входит

- **Реализация** `emulation/cooja/` — **P3.21**, **TBD**;
- **Установка** (Linux) — `gcc-msp430`, `arm-none-eabi`, `openjdk-25`, `./gradlew run`.

## Последствия

- `TODO.md` — `P3.21` (Cooja);
- `emulation/cooja/` — **создать** (TBD, P3.21).

## Ссылки

- [TODO.md](../../TODO.md) — P3.21 (Cooja);
- [ADR-0008: Фронтир](ADR-0008-frontier-term.md) — 7 состояний;
- [plate-b-soft.md](../system/plate-b-soft.md) — Thread/Zigbee (ESP32-C6/H2);
- [emulation/aig-502/README.md](../../emulation/aig-502/README.md) — AIG-502 эмулятор (Modbus, не WSN).
