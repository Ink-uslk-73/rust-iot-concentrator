//! src/network/card.rs - NetworkCard фасад (P3.47).

use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct LinkStats {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_drops: u64,
    pub tx_drops: u64,
}

#[derive(Debug, Clone)]
pub enum LinkEvent {
    Up { name: String },
    Down { name: String },
    New { name: String },
    Removed { name: String },
}

pub trait NetworkCard: Send + Sync + Debug {
    fn name(&self) -> &str;
    fn mac(&self) -> Option<[u8; 6]>;
    fn state(&self) -> LinkState;
    fn up(&self) -> Result<(), String>;
    fn down(&self) -> Result<(), String>;
    fn set_mtu(&self, mtu: u32) -> Result<(), String>;
    fn stats(&self) -> Result<LinkStats, String>;
}

pub fn create_all() -> Vec<Box<dyn NetworkCard>> {
    vec![]
}
