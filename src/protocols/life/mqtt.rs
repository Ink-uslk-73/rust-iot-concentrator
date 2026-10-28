use std::sync::Arc;
// src/protocols/life/mqtt.rs - MQTT (mqtt5 broker).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct MqttAdapter;

impl MqttAdapter {
    pub fn new() -> Self { Self }
}

impl Default for MqttAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for MqttAdapter {
    fn heartbeat_name(&self) -> Option<&'static str> { Some("PLATE-C") }
    fn name(&self) -> &'static str { "mqtt" }
    fn priority(&self) -> Priority { Priority::Life }

    fn start(&self, _publish: PublishFn, registry: Arc<crate::metrics::registry::Registry>) -> Result<(), String> {
        let reg = registry.clone();
        tokio::spawn(async move {
            loop {
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as f64).unwrap_or(0.0);
                reg.set_gauge("alive.PLATE-C", now);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }); Ok(()) }
}
