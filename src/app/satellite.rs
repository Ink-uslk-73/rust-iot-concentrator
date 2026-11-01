//! src/app/satellite.rs - SatelliteRegistry + Watchdog + heartbeat + protocols.

use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use crate::core::bus::Bus;
use crate::metrics::registry::Registry;
use crate::network;
use crate::protocols;
use crate::satellite::heartbeat::spawn_heartbeat;
use crate::satellite::watchdog::SatelliteWatchdog;
use crate::satellite::SatelliteRegistry;

pub struct SatelliteHandles {
    pub registry: Arc<SatelliteRegistry>,
}

pub fn init(bus: Bus) -> SatelliteHandles {
    let registry = Arc::new(SatelliteRegistry::new());
    let wd = SatelliteWatchdog::new(registry.clone(), bus);
    tokio::spawn(wd.run());
    SatelliteHandles { registry }
}

pub fn spawn_heartbeats(
    registry: Arc<SatelliteRegistry>,
    metrics_registry: Arc<Registry>,
    concentrator_cfg: Arc<RwLock<crate::config::ConcentratorConfig>>,
) {
    let cfg = concentrator_cfg.read().unwrap();
    for a in protocols::create_all(&cfg) {
        if let Some(sat_name) = a.heartbeat_name() {
            spawn_heartbeat(registry.clone(), sat_name, Arc::clone(&a), metrics_registry.clone(), Duration::from_secs(2), 3000);
        }
    }

    // network nodes (P3.35) — router, firewall, switch, bridge, gateway, router_soft
    for n in network::create_all() {
        if let Err(e) = n.start(metrics_registry.clone()) {
            tracing::warn!(node = n.name(), kind = ?n.kind(), err = %e, "network node start failed");
        } else {
            tracing::info!(node = n.name(), kind = ?n.kind(), "network node started");
        }
    }
}

