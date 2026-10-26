//! src/protocols/life/mqtt.rs - MQTT (mqtt5 broker).
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
    fn name(&self) -> &'static str { "mqtt" }
    fn priority(&self) -> Priority { Priority::Life }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
