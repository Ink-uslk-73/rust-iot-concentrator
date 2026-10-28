//! src/protocols/hard/mod.rs - HARD protocols (AIG-502, Debian 11).
//!
//! Modbus paths come from config/concentrator.toml [modbus].
//! Switch mode via UI: POST /api/v1/config/mode {"mode":"emulation"}

pub mod can;
pub mod etherip;
pub mod modbus;
pub mod modbus_tcp;
pub mod modbus_config;
pub mod opcua;
pub mod profinet;

use super::ProtocolAdapter;
use crate::config::ConcentratorConfig;

pub fn create_all(cfg: &ConcentratorConfig) -> Vec<Box<dyn ProtocolAdapter>> {
    let (config_path, registers_path) = cfg.modbus.current_paths();
    vec![
        Box::new(modbus::ModbusAdapter::new(config_path, registers_path)),
        Box::new(opcua::OpcUaAdapter::new()),
        Box::new(profinet::ProfinetAdapter::new()),
        Box::new(etherip::EtherIpAdapter::new()),
        Box::new(can::CanAdapter::new()),
    ]
}
