//! src/protocols/hard/mod.rs - HARD protocols (PLATE-A).
pub mod can;
pub mod etherip;
pub mod modbus;
pub mod opcua;
pub mod profinet;

use super::ProtocolAdapter;

pub fn create_all() -> Vec<Box<dyn ProtocolAdapter>> {
    vec![
        Box::new(modbus::ModbusAdapter::new()),
        Box::new(opcua::OpcUaAdapter::new()),
        Box::new(profinet::ProfinetAdapter::new()),
        Box::new(etherip::EtherIpAdapter::new()),
        Box::new(can::CanAdapter::new()),
    ]
}
