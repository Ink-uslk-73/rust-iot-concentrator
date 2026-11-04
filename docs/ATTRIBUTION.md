# Attribution — внешние SDK, утилиты, авторы

**Статус:** draft
**Назначение:** атрибуция внешних зависимостей (SDK, утилиты, драйверы), их авторов, лицензий и активных ссылок.

## Сетевой захват (P3.37)

| Компонент | Назначение | Автор / Организация | Активная ссылка | Лицензия |
|---|---|---|---|---|
| **Npcap SDK** (ZIP) | pcap crate: линковка wpcap.lib на Windows | The Nmap Project | https://npcap.com/#download | Npcap OEM / Free |
| **Npcap runtime** (EXE) | Захват пакетов на Windows (драйвер) | The Nmap Project | https://npcap.com/#download | Npcap Free / OEM |
| **libpcap** | pcap crate: захват на Linux/macOS | The Tcpdump Group | https://www.tcpdump.org/ | BSD-3-Clause |

**Важно (Windows):**
- Для сборки нужен **Npcap SDK** (ZIP, Lib\x64\wpcap.lib).
- Для запуска нужен **Npcap runtime** (installer .exe).
- **WinPcap** (C:\WpdPack) — НЕ подходит (LNK2019).
- Переменная: LIBPCAP_LIBDIR=C:\Npcap-SDK\Lib\x64

**Важно (Linux):**
- libpcap-dev (Debian/Ubuntu), libpcap-devel (Fedora).
- setcap cap_net_raw,cap_net_admin=eip path/to/bin (без root).

## Rust-крейты (P3.37)

| Крейт | Назначение | Автор / Организация | Активная ссылка |
|---|---|---|---|
| **pcap** 0.11 | Захват пакетов (libpcap/Npcap) | rust-pcap team | https://github.com/rust-pcap/pcap |
| **etherparse** 0.21 | Разбор Ethernet/IP/TCP/UDP (zero-allocation) | etherparse contributors | https://docs.rs/etherparse/0.21.0 |
| **netdev** 0.47 | Перечисление интерфейсов (MAC, MTU, state) | dignifiedquire | https://crates.io/crates/netdev/0.47.0 |
| **tokio** 1.40 | Async runtime | Tokio contributors | https://github.com/tokio-rs/tokio |

## WiFi (позже, P3.37 часть 2)

| Компонент | Назначение | Автор / Организация | Активная ссылка |
|---|---|---|---|
| **rfmon** | Ввод WiFi в monitor mode (Linux) | (уточнить) | (уточнить) |
| **Npcap** (опция "Support raw 802.11") | Захват 802.11 на Windows | The Nmap Project | https://npcap.com/guide/npcap-devguide.html |

## Открытые вопросы

- [ ] rfmon: автор/репозиторий — уточнить.
- [ ] simple_wifi: нужен ли (802.11 разбор)?
- [ ] Npcap OEM: нужна ли лицензия для распространения?
