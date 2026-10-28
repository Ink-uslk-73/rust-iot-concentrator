//! src/protocols/life/mod.rs
pub mod http;
pub mod mqtt;
pub mod polling;
pub mod terminal;

use std::sync::Arc;
use super::ProtocolAdapter;

pub fn create_all() -> Vec<Arc<dyn ProtocolAdapter>> {
    vec![
        Arc::new(mqtt::MqttAdapter::new()),
        Arc::new(http::HttpAdapter::new()),
        Arc::new(terminal::TerminalAdapter::new()),
        Arc::new(polling::PollingAdapter::new()),
    ]
}
