//! src/protocols/life/firewall.rs - Firewall (stub, P3.33).
use std::sync::Arc;
use std::time::Duration;
use crate::core::mode_switch::Priority;
use crate::metrics::registry::Registry;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct FirewallAdapter;

impl FirewallAdapter {
    pub fn new() -> Self { Self }
}

impl ProtocolAdapter for FirewallAdapter {
    fn name(&self) -> &'static str { "firewall" }
    fn priority(&self) -> Priority { Priority::Life }
    fn heartbeat_name(&self) -> Option<&'static str> { Some("PLATE-C") }
    fn start(&self, _publish: PublishFn, registry: Arc<Registry>) -> Result<(), String> {
        let reg = registry.clone();
        tokio::spawn(async move {
            loop {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as f64)
                    .unwrap_or(0.0);
                reg.set_gauge("alive.PLATE-C", now);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }
}
