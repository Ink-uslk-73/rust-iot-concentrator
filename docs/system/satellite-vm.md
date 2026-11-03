# Satellite VM — модель (4 VM + host)

**Статус:** accepted (P3.38).
**Связанные:** [network-nodes.md](network-nodes.md), [raw-abstraction.md](raw-abstraction.md), [pcie-topology.md](pcie-topology.md), [ADR-0018](../decisions/ADR-0018-wrx90e-platform.md).

## Модель

Не «3 VM по сателлитам», а 4 VM по ролям + host (CORE). PLATE-A/B/C — адаптеры
(в CORE), не VM. Как IBM (LPAR/z/VM): роли, изоляция, предсказуемость.

| Компонент | Роль | Где |
|---|---|---|
| **host (CORE)** | Bus, Dispatcher, API, SatelliteRegistry, alive.<PLATE>, Redis (systemd, watchdog, hot-path <1 мс), CORE-Raw (NVMe) | host (Debian + KVM/libvirt) |
| **DB-VM** | TimescaleDB (агрегаты 1s/1m/1h) | VM |
| **NET-VM** | NetworkNode (Router/Switch/Bridge/Gateway/Firewall), VLAN | VM |
| **BACKUP-VM** | Фоновый бэкап (снапшоты ≤32, ротация) | VM |
| **BOT-VM** | Automation runner: Lua + Python, FSM, диспетчер, планировщик, watchdog (внешний), багтрек (SQLite) | VM |
| **AUDIT-VM** | Аудит (read-only к CORE), LLM (локальный), отчёты | VM |
| **PLATE-A/B/C** | Адаптеры (поле, умный дом, каналы) | CORE (не VM) |

Итого: 5 VM (DB, NET, BACKUP, BOT, AUDIT) + host. PLATE-A/B/C — адаптеры, не VM (Оккам: не 7 VM).

## Диски (NVMe vs SATA)

| Назначение | Носитель | Почему |
|---|---|---|
| Raw (сателлит + CORE) | NVMe PCIe 5.0+ | hot-path, store & forward, <1 мс |
| TimescaleDB (DB-VM) | NVMe | агрегаты, 1s/1m/1h |
| BACKUP-VM | SATA (фоновый) | не RT, дешевле |
| Системные логи ОС (host) | SATA (1 диск) | /var/log, journald, не RT |

SATA — пятое колесо для RT, но ок для логов. Raw/TimescaleDB — только NVMe.

## Порядок записи

1. Raw — NVMe (сателлит + CORE), hot-path.
2. TimescaleDB — DB-VM (после Raw).
3. BACKUP-VM — фоновый (не блокирует hot-path), ротация ≤32.

## Redis (hot-path, в CORE)

- Redis — отдельный процесс (systemd), watchdog (внешний), hot-path <1 мс.
- При падении CORE — восстановление с сателлитов (Raw → CORE-Raw → Redis).
- Redis — оперативный кэш перед TimescaleDB, и состояние на момент падения.

## BOT-VM (automation runner)

- Lua + Python: Lua — таблицы, Python — логика (оба runtime).
- FSM: Idle → Running → Paused → Rollback → Idle.
- Диспетчер + планировщик (cron/interval, зависимости).
- Watchdog: внешний (host/плата с батарейкой), через API (/health, /status).
- Багтрек: SQLite (сейчас), Fossil — позже (если нужно).
- Ротация: ≤32 снапшота/записи журнала, ring buffer (старый перезаписывается).
- Изоляция: VLAN, отдельный MAC, доступ через API/CORE (не Raw, не PCIe).

## VM vs поды (K8s)

- HARD 10 мс: K8s jitter (планировщик, veth, softirq) ломает; VM — предсказуемо (vcpupin, numatune).
- Сеть: VM — bridge (без veth); K8s — Calico/Flannel (veth, оверхед).
- PCIe passthrough: VM — VFIO (штатно); K8s — SR-IOV + DPDK (привилегированные поды).
- Raw: VM — вне VM (NVMe); K8s — PVC (сложнее, риск потери).

## Рантайм (смотрим, не усложняем)

- Host (CORE): libvirt/qemu-kvm (Debian) — стабильно.
- DB/NET/BACKUP-VM: libvirt/qemu-kvm — persistent, bridge/VLAN.
- BOT-VM: libvirt/qemu-kvm сейчас.
- Kata 4.0 (runtime-rs/Dragonball) — смотрим, не берём (только если BOT-VM нужна эфемерность).
- rust-vmm (свой VMM) — смотрим, не берём (Оккам: libvirt работает).
- Rust в CORE (vm-manager, watchdog, bugtrack) — точечно, позже.

## Открытые вопросы

- [ ] DB-VM: TimescaleDB одна, или Redis тоже (если Redis не в CORE)?
- [ ] BACKUP-VM: SATA или NVMe (снапшоты ≤32)?
- [ ] BOT-VM: Kata/rust-vmm — когда (если понадобится)?
