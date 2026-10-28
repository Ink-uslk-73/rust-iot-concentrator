use std::sync::Arc;
// src/protocols/life/polling.rs - Polling (adaptive).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct PollingAdapter;

impl PollingAdapter {
    pub fn new() -> Self { Self }
}

impl Default for PollingAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for PollingAdapter {
    fn name(&self) -> &'static str { "polling" }
    fn priority(&self) -> Priority { Priority::Life }
    fn start(&self, _publish: PublishFn, _registry: Arc<crate::metrics::registry::Registry>) -> Result<(), String> { Ok(()) }
}
