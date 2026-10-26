//! src/protocols/life/mod.rs - LIFE protocols (PLATE-C).
pub mod http;
pub mod mqtt;
pub mod polling;
pub mod terminal;

use super::ProtocolAdapter;

pub fn create_all() -> Vec<Box<dyn ProtocolAdapter>> {
    vec![
        Box::new(mqtt::MqttAdapter::new()),
        Box::new(http::HttpAdapter::new()),
        Box::new(terminal::TerminalAdapter::new()),
        Box::new(polling::PollingAdapter::new()),
    ]
}
