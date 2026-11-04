//! src/raw/pcap_source.rs - PcapSource (Ethernet, P3.37).

use pcap::{Capture, Device};
use etherparse::{SlicedPacket, LinkSlice, NetSlice};
use super::source::RawSource;

pub struct PcapSource {
    name: String,
    cap: Capture<pcap::Active>,
}

impl PcapSource {
    pub fn open(iface: &str, filter: Option<&str>) -> Result<Self, String> {
        let dev = Device::list()
            .map_err(|e| e.to_string())?
            .into_iter()
            .find(|d| d.name == iface)
            .ok_or_else(|| format!("interface not found: {}", iface))?;
        let mut cap = Capture::from_device(dev)
            .map_err(|e| e.to_string())?
            .promisc(true)
            .snaplen(65535)
            .timeout(1000)
            .open()
            .map_err(|e| e.to_string())?;
        if let Some(f) = filter {
            cap.filter(f, true).map_err(|e| e.to_string())?;
        }
        Ok(Self { name: iface.to_string(), cap })
    }
}

impl RawSource for PcapSource {
    fn name(&self) -> &str { &self.name }
    fn next(&mut self) -> Result<Option<Vec<u8>>, String> {
        match self.cap.next_packet() {
            Ok(pkt) => Ok(Some(pkt.data.to_vec())),
            Err(pcap::Error::TimeoutExpired) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn mac_to_string(mac: &[u8; 6]) -> String {
    format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])
}

/// Разбор Ethernet-пакета (etherparse): MAC, EtherType (демо).
pub fn dissect_ethernet(data: &[u8]) -> Option<(String, String, String)> {
    let sliced = SlicedPacket::from_ethernet(data).ok()?;
    let (src, dst) = match &sliced.link {
        Some(LinkSlice::Ethernet2(eth)) => (
            mac_to_string(&eth.source()),
            mac_to_string(&eth.destination()),
        ),
        _ => return None,
    };
    let ethertype = match &sliced.net {
        Some(NetSlice::Ipv4(_)) => "IPv4",
        Some(NetSlice::Ipv6(_)) => "IPv6",
        Some(NetSlice::Arp(_)) => "ARP",
        None => "unknown",
    }.to_string();
    Some((src, dst, ethertype))
}
