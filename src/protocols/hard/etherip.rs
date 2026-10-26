//! src/protocols/hard/etherip.rs - EtherNet/IP (CIP).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct EtherIpAdapter;

impl EtherIpAdapter {
    pub fn new() -> Self { Self }
}

impl Default for EtherIpAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for EtherIpAdapter {
    fn name(&self) -> &'static str { "etherip" }
    fn priority(&self) -> Priority { Priority::Hard }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
