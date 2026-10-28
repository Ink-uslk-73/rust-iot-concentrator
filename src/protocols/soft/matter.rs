use std::sync::Arc;
// src/protocols/soft/matter.rs - Matter (controller).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct MatterAdapter;

impl MatterAdapter {
    pub fn new() -> Self { Self }
}

impl Default for MatterAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for MatterAdapter {
    fn heartbeat_name(&self) -> Option<&'static str> { Some("PLATE-B") }
    fn name(&self) -> &'static str { "matter" }
    fn priority(&self) -> Priority { Priority::Soft }

    fn start(&self, _publish: PublishFn, registry: Arc<crate::metrics::registry::Registry>) -> Result<(), String> {
        let reg = registry.clone();
        tokio::spawn(async move {
            loop {
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as f64).unwrap_or(0.0);
                reg.set_gauge("alive.PLATE-B", now);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }); Ok(()) }
}
