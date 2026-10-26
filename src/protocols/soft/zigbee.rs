//! src/protocols/soft/zigbee.rs - Zigbee (ziggurat).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct ZigbeeAdapter;

impl ZigbeeAdapter {
    pub fn new() -> Self { Self }
}

impl Default for ZigbeeAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for ZigbeeAdapter {
    fn name(&self) -> &'static str { "zigbee" }
    fn priority(&self) -> Priority { Priority::Soft }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
