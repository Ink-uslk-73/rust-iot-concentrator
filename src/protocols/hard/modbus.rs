//! src/protocols/hard/modbus.rs - Modbus TCP (10 ms).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct ModbusAdapter {
    #[allow(dead_code)]
    poll_ms: u64,
}

impl ModbusAdapter {
    pub fn new() -> Self {
        Self { poll_ms: 10 }
    }
}

impl Default for ModbusAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for ModbusAdapter {
    fn name(&self) -> &'static str { "modbus" }
    fn priority(&self) -> Priority { Priority::Hard }
    fn start(&self, _publish: PublishFn) -> Result<(), String> {
        Ok(())
    }
}
