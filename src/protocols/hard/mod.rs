//! src/protocols/hard/mod.rs - HARD protocols (AIG-502, Debian 11).
pub mod can;
pub mod etherip;
pub mod modbus;
pub mod modbus_config;
pub mod opcua;
pub mod profinet;

use super::ProtocolAdapter;

pub fn create_all() -> Vec<Box<dyn ProtocolAdapter>> {
    vec![
        Box::new(modbus::ModbusAdapter::new(
            "/var/thingspro/data/config.json",
            "/var/thingspro/data/registers.json",
        )),
        Box::new(opcua::OpcUaAdapter::new()),
        Box::new(profinet::ProfinetAdapter::new()),
        Box::new(etherip::EtherIpAdapter::new()),
        Box::new(can::CanAdapter::new()),
    ]
}
