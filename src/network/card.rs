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

pub trait NetworkCard: Send + Sync + Debug {
    /// MAC-адрес (например, "aa:bb:cc:dd:ee:ff").
    fn mac_address(&self) -> &str;
    /// Скорость (Мбит/с), 0 если неизвестна.
    fn speed_mbps(&self) -> u32;
    /// Интерфейс поднят (operational state).
    fn is_up(&self) -> bool;
    /// MTU (обычно 1500).
    fn mtu(&self) -> u16;
    /// Состояние (Up/Down/Unknown).
    fn state(&self) -> LinkState {
        if self.is_up() { LinkState::Up } else { LinkState::Down }
    }
    /// Поднять интерфейс.
    fn up(&self) -> Result<(), String>;
    /// Опустить интерфейс.
    fn down(&self) -> Result<(), String>;
    /// Установить MTU.
    fn set_mtu(&self, mtu: u16) -> Result<(), String>;
    /// Статистика (rx/tx bytes/packets/errors/drops).
    fn stats(&self) -> Result<LinkStats, String>;
    /// Описание (default: "NIC <mac> @ <speed> Мбит/с").
    fn description(&self) -> String {
        format!("NIC {} @ {} Мбит/с", self.mac_address(), self.speed_mbps())
    }
}

pub fn create_all() -> Vec<Box<dyn NetworkCard>> {
    vec![]
}
