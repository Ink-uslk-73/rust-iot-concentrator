//! src/protocols/hard/can.rs - CAN (socketcan).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct CanAdapter;

impl CanAdapter {
    pub fn new() -> Self { Self }
}

impl Default for CanAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for CanAdapter {
    fn name(&self) -> &'static str { "can" }
    fn priority(&self) -> Priority { Priority::Hard }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
