//! src/network/linux_card.rs - LinuxCard (nlink, P3.47 часть 3).

use nlink::netlink::{Connection, Route};
use super::card::{NetworkCard, LinkStats};

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
            let all = conn.get_stats64_all().await.map_err(|e| e.to_string())?;
            let s = all.iter().find(|s| s.ifindex == self.index)
                .ok_or_else(|| "stats not found".to_string())?;
            Ok(LinkStats {
                rx_bytes: s.rx_bytes, tx_bytes: s.tx_bytes,
                rx_packets: s.rx_packets, tx_packets: s.tx_packets,
                rx_errors: s.rx_errors, tx_errors: s.tx_errors,
                rx_drops: s.rx_dropped, tx_drops: s.tx_dropped,
            })
        })
    }
}
