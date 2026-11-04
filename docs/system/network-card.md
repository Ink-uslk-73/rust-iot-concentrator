# NetworkCard — фасад (P3.47)

**Статус:** accepted (P3.47).
**Связанные:** [network-nodes.md](network-nodes.md), [network-topology.md](network-topology.md), [raw-abstraction.md](raw-abstraction.md).

## Идея

Один trait NetworkCard (фасад) — единый интерфейс к сетевым картам (Ethernet, WiFi, роутер).
Реализации: nlink (Linux), netconfig-rs (cross-platform), pcap (захват).

## trait NetworkCard

- name() -> &str; mac() -> MacAddr; state() -> LinkState;
- up()/down(); set_mtu(u32);
- stats() -> LinkStats (rx/tx bytes/packets/errors/drops);
- events() -> Stream<LinkEvent> (up/down, new/removed).

## Реализации

| Реализация | Crate | Что |
|---|---|---|
| LinuxCard | nlink 0.25 | netlink: links, ethtool stats, link events, nl80211 (WiFi) |
| CrossPlatformCard | netconfig-rs 0.1 | Linux/Windows: interfaces, IP, state |
| CaptureSource | pcap 0.11 | promiscuous, BPF, savefile (RawSource) |

## runact (опционально, P3.48)

- Каждая карта — актор (runact: Actor trait, supervision, timers, backpressure) ;
- Сбои карты -> супервизия (restart, backoff);
- ractor — меморандум (магистрали, cluster, позже) .

## Открытые вопросы

- [ ] runact: акторы для карт — когда (P3.48)?
- [ ] LinuxCard: ethtool stats — какие поля (rx/tx, errors, drops)?
- [ ] WiFi: nl80211 (nlink) или rfmon + pcap?
