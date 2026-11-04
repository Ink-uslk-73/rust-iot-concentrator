//! src/network/bridge.rs - Bridge (L2, stub, P3.35).
use std::sync::Arc;
use std::time::Duration;
use crate::metrics::registry::Registry;
use super::{NetworkNode, NodeKind};

pub struct BridgeNode;

impl BridgeNode {
    pub fn new() -> Self { Self }
}

impl Default for BridgeNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkNode for BridgeNode {
    fn name(&self) -> &'static str { "bridge" }
    fn kind(&self) -> NodeKind { NodeKind::Bridge }
    fn start(&self, registry: Arc<Registry>) -> Result<(), String> {
        let reg = registry.clone();
        tokio::spawn(async move {
            loop {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as f64)
                    .unwrap_or(0.0);
                reg.set_gauge("alive.bridge", now);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }
}
