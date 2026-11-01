//! src/network/router_soft.rs - Router (soft, stub, P3.35).
use std::sync::Arc;
use std::time::Duration;
use crate::metrics::registry::Registry;
use super::{NetworkNode, NodeKind};

pub struct RouterSoftNode;

impl RouterSoftNode {
    pub fn new() -> Self { Self }
}

impl NetworkNode for RouterSoftNode {
    fn name(&self) -> &'static str { "router_soft" }
    fn kind(&self) -> NodeKind { NodeKind::RouterSoft }
    fn start(&self, registry: Arc<Registry>) -> Result<(), String> {
        let reg = registry.clone();
        tokio::spawn(async move {
            loop {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as f64)
                    .unwrap_or(0.0);
                reg.set_gauge("alive.router_soft", now);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }
}
