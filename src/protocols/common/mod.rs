//! src/protocols/common/mod.rs - общее для всех сателлитов (P3.36).

use std::sync::Arc;
use crate::core::bus::{Bus, BusMessage, BusPayload};
use crate::core::mode_switch::Priority;
use crate::metrics::registry::Registry;

pub type PublishFn = Arc<dyn Fn(Priority, f64, &str) + Send + Sync>;

pub trait ProtocolAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn priority(&self) -> Priority;
    fn start(&self, publish: PublishFn, registry: Arc<Registry>) -> Result<(), String>;
    /// Имя сателлита ("PLATE-A", "PLATE-B", "PLATE-C") для heartbeat.
    fn heartbeat_name(&self) -> Option<&'static str> { None }
    /// Проверка живости: читает alive.<name> из Registry и проверяет свежесть.
    fn health_check(&self, registry: &Registry, timeout_ms: u64) -> bool {
        let Some(name) = self.heartbeat_name() else { return true; };
        let key = format!("alive.{}", name);
        let Some(ts) = registry.get_gauge(&key) else { return false; };
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0);
        (now_ms - ts) < timeout_ms as f64
    }
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
