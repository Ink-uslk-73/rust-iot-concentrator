//! src/network/linux_card.rs - LinuxCard (nlink, P3.47 часть 3).

use nlink::{new_connection, Handle};
use super::card::{NetworkCard, LinkStats};

pub struct LinuxCard {
    pub name: String,
    pub mac: String,
    pub index: u32,
    pub up: bool,
    pub mtu: u16,
}

impl LinuxCard {
    pub fn from_link(link: &nlink::Handle) -> Option<Self> {
        None // заглушка: заполним после проверки API
    }
}

impl NetworkCard for LinuxCard {
    fn mac_address(&self) -> &str { &self.mac }
    fn speed_mbps(&self) -> u32 { 0 }
    fn is_up(&self) -> bool { self.up }
    fn mtu(&self) -> u16 { self.mtu }
    fn up(&self) -> Result<(), String> {
        let (conn, handle, _) = new_connection().map_err(|e| e.to_string())?;
        let mut handle = handle;
        handle.link().set_up(self.index).execute().map_err(|e| e.to_string())?;
        Ok(())
    }
    fn down(&self) -> Result<(), String> {
        let (conn, handle, _) = new_connection().map_err(|e| e.to_string())?;
        let mut handle = handle;
        handle.link().set_down(self.index).execute().map_err(|e| e.to_string())?;
        Ok(())
    }
    fn set_mtu(&self, _mtu: u16) -> Result<(), String> { Err("nlink: set_mtu (позже)".into()) }
    fn stats(&self) -> Result<LinkStats, String> { Err("nlink: stats (позже)".into()) }
}
