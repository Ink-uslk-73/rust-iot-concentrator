use std::sync::Arc;
// src/protocols/hard/profinet.rs - PROFINET (DPI).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct ProfinetAdapter;

impl ProfinetAdapter {
    pub fn new() -> Self { Self }
}

impl Default for ProfinetAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for ProfinetAdapter {
    fn name(&self) -> &'static str { "profinet" }
    fn priority(&self) -> Priority { Priority::Hard }
    fn start(&self, _publish: PublishFn, _registry: Arc<crate::metrics::registry::Registry>) -> Result<(), String> { Ok(()) }
}
