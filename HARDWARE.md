# HARDWARE.md - Телекоммуникационный концентратор v4.0

## 1. Платформа
| Параметр | Значение |
|----------|----------|
| Модель | Gigabyte WG-C2.R2H.H224-A2 (WIT VV) |
| Форм-фактор | 2U, 24x HDD 2.5" |
| Управление | IPMI/BMC (VLAN 10) |

## 2. CPU
| Параметр | Значение |
|----------|----------|
| Сокеты | 2x AMD EPYC 7002 |
| Ядра/потоки | 64/128 на сокет |
| NUMA | 2 узла |

## 3. Память
| Параметр | Значение |
|----------|----------|
| Объём | >= 512 ГБ |
| NUMA-политика | per-thread affinity + mbind |

## 4. Платы расширения
### PLATE-A - Industrial (HARD)
- Modbus, OPC UA, PROFINET, EtherNet/IP, CAN; ядра 0-7, SCHED_FIFO

### PLATE-B - Smart Home (SOFT)
- Matter, Zigbee, Thread, BLE; ядра 8-31

### PLATE-C - Router/Gateway (LIFE)
- Ethernet, Wi-Fi, VPN; ядра 64-95

## 5. Хранилище
| Параметр | Значение |
|----------|----------|
| Диски | 24x HDD 2.5" SAS/SATA |
| Раскладка | TimescaleDB / Loki / coredump |
| Ядра | 96-127 (node 1) |

## 6. Сеть (VLAN)
| VLAN | Назначение | Доступ |
|------|------------|--------|
| 10 | MANAGEMENT | 22, IPMI |
| 20 | TELEMETRY | 9091 |
| 30 | TABLET | 8443, 8444 (mTLS) |
| 40 | INDUSTRIAL | внутренний |
| 50 | SMARTHOME | внутренний |
| 60 | ROUTER | внешний |

## 7. Планшет
| Параметр | Значение |
|----------|----------|
| Подключение | Wi-Fi VLAN 30, WPA3-Enterprise |
| Аутентификация | mTLS |
| Порты | 8443 (дашборд), 8444 (терминал) |
| UI | Svelte + htmx |

## 8. GRUB
```
GRUB_CMDLINE_LINUX="isolcpus=0-7,64-71 nohz_full=0-7,64-71 rcu_nocbs=0-7,64-71"
```

## 9. Связь железа с софтом
| Домен | NUMA | Ядра | Планировщик | Память |
|-------|------|------|-------------|--------|
| HARD | 0 | 0-7 | SCHED_FIFO 90 | node 0, mlockall |
| SOFT | 0 | 8-31 | SCHED_OTHER | node 0 |
| CORE | 0 | 32-47 | SCHED_OTHER | node 0 |
| LIFE | 1 | 64-95 | SCHED_OTHER | node 1 |
| STORAGE | 1 | 96-127 | SCHED_OTHER | node 1 |

> **Терминология:** "Концентратор" здесь - **Data Concentrator** (IIoT),
> агрегатор данных с N устройств -> 1 uplink. Не Ethernet hub (L1, устаревший).
> См. docs/glossary.md.