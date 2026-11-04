//! src/network/netdev_card.rs - NetworkCard через netdev (P3.47).

use netdev::get_interfaces;
use super::card::{NetworkCard, LinkStats};

#[derive(Debug)]
pub struct NetdevCard {
    pub name: String,
    pub mac: String,
    pub speed: u32,
    pub up: bool,
    pub mtu: u16,
}

impl NetworkCard for NetdevCard {
    fn mac_address(&self) -> &str { &self.mac }
    fn speed_mbps(&self) -> u32 { self.speed }
    fn is_up(&self) -> bool { self.up }
    fn mtu(&self) -> u16 { self.mtu }
    fn up(&self) -> Result<(), String> { Err("netdev: up/down via OS (not impl)".into()) }
    fn down(&self) -> Result<(), String> { Err("netdev: up/down via OS (not impl)".into()) }
    fn set_mtu(&self, _mtu: u16) -> Result<(), String> { Err("netdev: set_mtu via OS (not impl)".into()) }
    fn stats(&self) -> Result<LinkStats, String> { Err("netdev: stats via OS (not impl)".into()) }
}

pub fn create_all() -> Vec<Box<dyn NetworkCard>> {
    get_interfaces()
        .into_iter()
        .map(|iface| {
            Box::new(NetdevCard {
                name: iface.name.clone(),
                mac: iface.mac_addr.map(|m| m.to_string()).unwrap_or_default(),
                speed: 0,
                up: iface.is_up(),
                mtu: iface.mtu.unwrap_or(1500) as u16,
            }) as Box<dyn NetworkCard>
        })
        .collect()
}
