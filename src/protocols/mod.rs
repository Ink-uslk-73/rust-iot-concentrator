//! src/protocols/mod.rs - protocol adapters.

pub mod hard;
pub mod life;
pub mod soft;

use std::sync::Arc;
use crate::config::ConcentratorConfig;
use crate::core::bus::{Bus, BusMessage, BusPayload};
use crate::core::mode_switch::Priority;

pub type PublishFn = Arc<dyn Fn(Priority, f64, &str) + Send + Sync>;

pub trait ProtocolAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn priority(&self) -> Priority;
    fn start(&self, publish: PublishFn) -> Result<(), String>;
}

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

pub fn create_all(cfg: &ConcentratorConfig) -> Vec<Box<dyn ProtocolAdapter>> {
    let mut out: Vec<Box<dyn ProtocolAdapter>> = Vec::new();
    out.extend(hard::create_all(cfg));
    out.extend(soft::create_all());
    out.extend(life::create_all());
    out
}
