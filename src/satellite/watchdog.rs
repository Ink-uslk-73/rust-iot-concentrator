//! src/satellite/watchdog.rs - Satellite watchdog (timeout -> alert).

use std::sync::Arc;
use std::time::Duration;
use tracing::warn;

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;
use super::{SatelliteRegistry, SatelliteState};

pub struct SatelliteWatchdog {
    registry: Arc<SatelliteRegistry>,
    bus: Bus,
    timeout: Duration,
    threshold: u32,
    interval: Duration,
}

impl SatelliteWatchdog {
    pub fn new(registry: Arc<SatelliteRegistry>, bus: Bus) -> Self {
        Self {
            registry,
            bus,
            timeout: Duration::from_secs(10),
            threshold: 3,
            interval: Duration::from_millis(500),
        }
    }

    pub async fn run(self) {
        let mut tick = tokio::time::interval(self.interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tick.tick().await;
            let statuses = self.registry.check(self.timeout, self.threshold);
            for s in statuses {
                if s.state == SatelliteState::Stopped.as_str() {
                    warn!(satellite = %s.name, "satellite stopped");
                    let _ = self.bus.diagnostics.try_send(BusMessage::new(
                        "satellite-watchdog",
                        Priority::Hard,
                        BusPayload::Diagnostic {
                            level: DiagLevel::Critical,
                            component: s.name.clone(),
                            message: format!("satellite stopped ({} ms)", s.last_heartbeat_ms),
                        },
                    ));
                }
            }
        }
    }
}
