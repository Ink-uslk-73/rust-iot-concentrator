use std::sync::Arc;
// src/protocols/hard/opcua.rs - OPC UA (subscription).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct OpcUaAdapter;

impl OpcUaAdapter {
    pub fn new() -> Self { Self }
}

impl Default for OpcUaAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for OpcUaAdapter {
    fn name(&self) -> &'static str { "opcua" }
    fn priority(&self) -> Priority { Priority::Hard }
    fn start(&self, _publish: PublishFn, _registry: Arc<crate::metrics::registry::Registry>) -> Result<(), String> { Ok(()) }
}
