//! src/protocols/mod.rs - protocol adapters.
//!
//! Trait ProtocolAdapter: start() runs protocol loop, publishes to Bus.
//! Each adapter gets a Bus (clone) and a priority.

pub mod hard;
pub mod soft;
pub mod life;

use std::sync::Arc;
use crate::core::bus::{Bus, BusMessage, BusPayload};
use crate::core::mode_switch::Priority;

/// Callback for publishing telemetry to Bus.
pub type PublishFn = Arc<dyn Fn(Priority, f64, &str) + Send + Sync>;

/// Protocol adapter.
pub trait ProtocolAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn priority(&self) -> Priority;
    /// Start the loop. Returns when stopped or on error.
    fn start(&self, publish: PublishFn) -> Result<(), String>;
}

/// Create publish callback from Bus.
pub fn make_publish(bus: Bus, sensor_prefix: &'static str) -> PublishFn {
    Arc::new(move |prio, value, unit| {
        let bus = bus.clone();
        let unit = unit.to_string();
        tokio::spawn(async move {
            let _ = bus.send(BusMessage::new(
                sensor_prefix,
                prio,
                BusPayload::Telemetry {
                    sensor_id: format!("{}-x", sensor_prefix),
                    value,
                    unit,
                    timestamp_ms: 0,
                },
            )).await;
        });
    })
}

/// Create all adapters (stub for now).
pub fn create_all() -> Vec<Box<dyn ProtocolAdapter>> {
    let mut out: Vec<Box<dyn ProtocolAdapter>> = Vec::new();
    out.extend(hard::create_all());
    out.extend(soft::create_all());
    out.extend(life::create_all());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_all_returns_some() {
        let v = create_all();
        assert!(!v.is_empty());
    }
}
