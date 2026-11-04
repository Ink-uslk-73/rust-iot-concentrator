//! src/network/mod.rs - NetworkNode pattern (P3.35).
//! Router (real/soft), Switch, Bridge, Gateway, Firewall — общее в trait,
//! отличия — в плагинах (оверлей, протоколы), конфиге (TOML), метриках (UI).

use std::sync::Arc;
use crate::metrics::registry::Registry;

pub mod bridge;
pub mod card;
pub mod firewall;
pub mod gateway;
pub mod router;
pub mod router_soft;
pub mod switch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    RouterReal,
    RouterSoft,
    Switch,
    Bridge,
    Gateway,
    Firewall,
}

pub trait NetworkNode: Send + Sync {
    fn name(&self) -> &'static str;
    fn kind(&self) -> NodeKind;
    fn start(&self, registry: Arc<Registry>) -> Result<(), String>;
    fn health_check(&self, registry: &Registry, timeout_ms: u64) -> bool {
        let key = format!("alive.{}", self.name());
        let Some(ts) = registry.get_gauge(&key) else { return false; };
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0);
        (now_ms - ts) < timeout_ms as f64
    }
}

pub fn create_all() -> Vec<Arc<dyn NetworkNode>> {
    vec![
        Arc::new(router::RouterNode::new()),
        Arc::new(firewall::FirewallNode::new()),
        Arc::new(switch::SwitchNode::new()),
        Arc::new(bridge::BridgeNode::new()),
        Arc::new(gateway::GatewayNode::new()),
        Arc::new(router_soft::RouterSoftNode::new()),
    ]
}
