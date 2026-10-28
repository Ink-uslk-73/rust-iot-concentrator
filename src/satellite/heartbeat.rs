//! src/satellite/heartbeat.rs - Periodic heartbeat for satellites.

use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

use super::SatelliteRegistry;

pub fn spawn_heartbeat(
    registry: Arc<SatelliteRegistry>,
    name: &'static str,
    interval: Duration,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tick.tick().await;
            registry.heartbeat(name);
            debug!(satellite = name, "heartbeat");
        }
    })
}
