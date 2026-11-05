//! src/network/linux_card.rs - LinuxCard (nlink, P3.47 часть 3).

use nlink::netlink::{Connection, Route};
use super::card::{NetworkCard, LinkStats};

#[derive(Debug)]
pub struct LinuxCard {
    pub name: String,
    pub mac: String,
    pub index: i32,
    pub up: bool,
    pub mtu: u16,
}

impl NetworkCard for LinuxCard {
    fn mac_address(&self) -> &str { &self.mac }
    fn speed_mbps(&self) -> u32 { 0 }
    fn is_up(&self) -> bool { self.up }
    fn mtu(&self) -> u16 { self.mtu }
    fn up(&self) -> Result<(), String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async {
            let conn = Connection::<Route>::new().map_err(|e| e.to_string())?;
            conn.set_link_up(&self.name).await.map_err(|e| e.to_string())
        })
    }
    fn down(&self) -> Result<(), String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async {
            let conn = Connection::<Route>::new().map_err(|e| e.to_string())?;
            conn.set_link_down(&self.name).await.map_err(|e| e.to_string())
        })
    }
    fn set_mtu(&self, mtu: u16) -> Result<(), String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async {
            let conn = Connection::<Route>::new().map_err(|e| e.to_string())?;
            conn.set_link_mtu(&self.name, mtu as u32).await.map_err(|e| e.to_string())
        })
    }
    fn stats(&self) -> Result<LinkStats, String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async {
            let conn = Connection::<Route>::new().map_err(|e| e.to_string())?;
            let links = conn.get_links().await.map_err(|e| e.to_string())?;
            let link = links.iter()
                .find(|l| l.ifindex() == self.index as u32)
                .ok_or_else(|| "stats not found".to_string())?;
            let s = link.stats().ok_or_else(|| "no stats".to_string())?;
            Ok(LinkStats {
                rx_bytes: s.rx_bytes() as u64,
                tx_bytes: s.tx_bytes() as u64,
                rx_packets: s.rx_packets() as u64,
                tx_packets: s.tx_packets() as u64,
                rx_errors: s.rx_errors() as u64,
                tx_errors: s.tx_errors() as u64,
                rx_drops: s.rx_dropped() as u64,
                tx_drops: s.tx_dropped() as u64,
            })
        })
    }
}