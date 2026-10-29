# Программные роутер и шлюз

Роутер и шлюз — **программные**, не отдельные железки. Работают на доменах
сателлита. P2P не используется (P2P у концентратора).

## Домены

| Домен | Роль | Обоснование |
|---|---|---|
| **PLATE-A (Hard)** | Роутер | ближе к полевым шинам, Modbus RTU/TCP, минимальная задержка |
| **PLATE-B (Soft)** | Вычисления, AI | не участвует в маршрутизации |
| **PLATE-C (Life)** | Шлюз, Modbus TCP slave, failover | P3.19: AIG-502 failover, snapshot из Redis |

## Роутер (PLATE-A)

- Маршрутизация Modbus RTU ↔ Modbus TCP.
- Приоритизация трафика (Bus `Priority::Hard/Soft/Life`).
- Fallback: при потере PLATE-A — PLATE-C берёт на себя (failover P3.19).

## Шлюз (PLATE-C)

- Modbus TCP slave (P3.19).
- Snapshot состояния из Redis при failover.
- Heartbeat через `alive.<PLATE>` (P2.20c).

## Открытые вопросы

- [ ] Подтвердить распределение A/C (роутер на A, шлюз на C).
- [ ] Нужен ли отдельный виртуальный интерфейс для шлюза (VLAN)?
- [ ] Адресация Modbus TCP: статическая или DHCP?

