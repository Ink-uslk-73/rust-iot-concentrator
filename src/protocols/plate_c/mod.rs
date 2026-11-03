//! src/protocols/plate_c/mod.rs - PLATE-C (LIFE): каналы (P3.36).

pub mod http;
pub mod mqtt;
pub mod polling;
pub mod terminal;

use std::sync::Arc;
use super::common::ProtocolAdapter;

pub fn create_all() -> Vec<Arc<dyn ProtocolAdapter>> {
    vec![
        Arc::new(mqtt::MqttAdapter::new()),
        Arc::new(http::HttpAdapter::new()),
        Arc::new(terminal::TerminalAdapter::new()),
        Arc::new(polling::PollingAdapter::new()),
    ]
}
