//! src/app/bus.rs - Bus + metrics + registry + self-telemetry.

use std::sync::Arc;
use std::time::Duration;

use crate::core::bus::{Bus, BusReceivers};
use crate::metrics::OverheadMetrics;
use crate::metrics::registry::Registry;
use crate::metrics::self_telemetry::SelfTelemetry;

pub struct BusHandles {
    pub bus: Bus,
    pub bus_rx: BusReceivers,
    pub metrics: Arc<OverheadMetrics>,
    pub registry: Arc<Registry>,
}

pub fn init() -> BusHandles {
    let metrics = Arc::new(OverheadMetrics::new());
    let registry = Arc::new(Registry::new());
    let mut self_tel = SelfTelemetry::new();
    let registry_bg = registry.clone();
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(2));
        loop { tick.tick().await; self_tel.publish(&registry_bg); }
    });

    let (bus, bus_rx) = Bus::new(256, 4096, 128, 128, metrics.clone());
    BusHandles { bus, bus_rx, metrics, registry }
}

