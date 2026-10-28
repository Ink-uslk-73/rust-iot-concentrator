use std::sync::Arc;
// src/protocols/soft/ble.rs - BLE (bluetooth-ctl).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct BleAdapter;

impl BleAdapter {
    pub fn new() -> Self { Self }
}

impl Default for BleAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for BleAdapter {
    fn name(&self) -> &'static str { "ble" }
    fn priority(&self) -> Priority { Priority::Soft }
    fn start(&self, _publish: PublishFn, _registry: Arc<crate::metrics::registry::Registry>) -> Result<(), String> { Ok(()) }
}
