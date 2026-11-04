//! src/network/firewall.rs - Firewall (stub, P3.35).
use std::sync::Arc;
use std::time::Duration;
use crate::metrics::registry::Registry;
use super::{NetworkNode, NodeKind};

pub struct FirewallNode;

impl FirewallNode {
    pub fn new() -> Self { Self }
}

impl Default for FirewallNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkNode for FirewallNode {
    fn name(&self) -> &'static str { "firewall" }
    fn kind(&self) -> NodeKind { NodeKind::Firewall }
    fn start(&self, registry: Arc<Registry>) -> Result<(), String> {
        let reg = registry.clone();
        tokio::spawn(async move {
            loop {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as f64)
                    .unwrap_or(0.0);
                reg.set_gauge("alive.firewall", now);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }
}
