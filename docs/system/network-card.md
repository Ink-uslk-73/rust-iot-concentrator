# NetworkCard — фасад (P3.47/P3.48)

**Статус:** accepted (P3.47, P3.48).
**Связанные:** [network-nodes.md](network-nodes.md), [network-topology.md](network-topology.md), [raw-abstraction.md](raw-abstraction.md), [ATTRIBUTION.md](../ATTRIBUTION.md).

## Идея

Один trait NetworkCard (фасад) — единый интерфейс к сетевым картам (Ethernet, WiFi, роутер).
Реализации: netdev (cross-platform, read-only), nlink (Linux, up/down), pcap (захват), rfmon (WiFi monitor, Linux).

## trait NetworkCard

- mac_address() -> &str; speed_mbps() -> u32; is_up() -> bool; mtu() -> u16;
- state() -> LinkState (Up/Down/Unknown, default из is_up);
- up()/down() -> Result<(), String>; set_mtu(u16) -> Result<(), String>;
- stats() -> Result<LinkStats, String> (rx/tx bytes/packets/errors/drops);
- description() -> String (default: "NIC <mac> @ <speed> Мбит/с").

## Реализации

| Реализация | Crate | Где | Что |
|---|---|---|---|
| NetdevCard | netdev 0.47 | Windows/Linux | read-only: MAC, MTU, oper state; up/down/set_mtu/stats — Err (заглушка) |
| LinuxCard | nlink 0.25 | Linux only | netlink: up/down (set_mtu/stats — позже) |
| PcapSource | pcap 0.11 + etherparse 0.21 | Windows/Linux | promiscuous, BPF, live capture; dissect_ethernet (MAC, EtherType) |
| FileSource | pcap 0.11 | Windows/Linux | pcap Offline (файл), NoMorePackets = None |
| WifiSource | rfmon 0.1 + pcap 0.11 | Linux only | monitor mode (RAII), 802.11 capture |
| CardActor | runact 1.3 | Linux only | актор: CardMsg (Up/Down/Stats), ActorRef |

## runact / ractor

- runact (Linux only): CardActor, Actor trait, ActorRef; epoll; Windows — без акторов.
- ractor — альтернатива (Tokio, cross-platform), позже; cluster (ractor_cluster) — меморандум (магистрали).

## Открытые вопросы

- [ ] LinuxCard: set_mtu/stats (nlink API, позже).
- [ ] NetdevCard: up/down/set_mtu/stats (Windows API, позже).
- [ ] WifiSource: разбор 802.11 (Radiotap/Frame), simple_wifi?
- [ ] runact: supervision (restart/backoff), P3.48 (позже).
- [ ] ractor: cross-platform акторы (Windows), когда?
- [ ] P3.49: деплой, канарейки (нечётные — сырые, чётные — стабильные), сторож версий (раз в сутки).
