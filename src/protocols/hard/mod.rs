//! src/protocols/hard/mod.rs
pub mod can;
pub mod etherip;
pub mod modbus;
pub mod modbus_config;
pub mod modbus_tcp;
pub mod opcua;
pub mod profinet;

use std::sync::Arc;
use super::ProtocolAdapter;
use crate::config::ConcentratorConfig;

pub fn create_all(cfg: &ConcentratorConfig) -> Vec<Arc<dyn ProtocolAdapter>> {
    let (config_path, registers_path) = cfg.modbus.current_paths();
    vec![
        Arc::new(modbus::ModbusAdapter::new(config_path, registers_path)),
        Arc::new(modbus_tcp::ModbusTcpServer::new(502)),
        Arc::new(opcua::OpcUaAdapter::new()),
        Arc::new(profinet::ProfinetAdapter::new()),
        Arc::new(etherip::EtherIpAdapter::new()),
        Arc::new(can::CanAdapter::new()),
    ]
}
