//! src/satellite/heartbeat.rs - Heartbeat with real health_check.

use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

use crate::metrics::registry::Registry;
use crate::protocols::ProtocolAdapter;
use super::SatelliteRegistry;

pub fn spawn_heartbeat(
    registry: Arc<SatelliteRegistry>,
    name: &'static str,
    adapter: Arc<dyn ProtocolAdapter>,
    metrics: Arc<Registry>,
    interval: Duration,
    timeout_ms: u64,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tick.tick().await;
            if adapter.health_check(&metrics, timeout_ms) {
                registry.heartbeat(name);
                debug!(satellite = name, "heartbeat");
            } else {
                debug!(satellite = name, "health_check=false (alive stale)");
            }
        }
    })
}
