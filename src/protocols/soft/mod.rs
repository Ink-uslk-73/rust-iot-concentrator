//! src/protocols/soft/mod.rs - SOFT protocols (PLATE-B).
pub mod ble;
pub mod matter;
pub mod thread;
pub mod zigbee;

use super::ProtocolAdapter;

pub fn create_all() -> Vec<Box<dyn ProtocolAdapter>> {
    vec![
        Box::new(matter::MatterAdapter::new()),
        Box::new(zigbee::ZigbeeAdapter::new()),
        Box::new(thread::ThreadAdapter::new()),
        Box::new(ble::BleAdapter::new()),
    ]
}
