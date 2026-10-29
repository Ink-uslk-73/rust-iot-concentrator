//! src/transport/mod.rs - NUMA-aware transport (shmem/vsock/nop).
//!
//! - shmem: HARD -> node 0 (SPSC ring, lock-free).
//! - vsock: CORE <-> SOFT/LIFE/STORAGE (cross-node).
//! - nop: fallback (Windows, tests).
//!
//! NOT Arc<Mutex> between domains - serialization + transport.

pub mod nop;
pub mod shmem;
pub mod vsock;
pub mod reconnect;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportMessage {
    pub source: String,
    pub priority: u8,
    pub payload: Vec<u8>,
}

pub trait Transport: Send + Sync {
    fn send(&self, msg: TransportMessage) -> Result<(), String>;
    fn recv(&self) -> Option<TransportMessage>;
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportKind {
    Shmem,
    Vsock,
    Nop,
}

impl TransportKind {
    pub fn from_str(s: &str) -> Self {
        match s {
            "shmem" => TransportKind::Shmem,
            "vsock" => TransportKind::Vsock,
            _ => TransportKind::Nop,
        }
    }
}
pub mod frame;
pub mod peer_link;
