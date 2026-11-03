//! src/protocols/plate_b/mod.rs - PLATE-B (SOFT): умный дом (P3.36).

pub mod ble;
pub mod matter;
pub mod thread;
pub mod zigbee;

use std::sync::Arc;
use super::common::ProtocolAdapter;

pub fn create_all() -> Vec<Arc<dyn ProtocolAdapter>> {
    vec![
        Arc::new(matter::MatterAdapter::new()),
        Arc::new(zigbee::ZigbeeAdapter::new()),
        Arc::new(thread::ThreadAdapter::new()),
        Arc::new(ble::BleAdapter::new()),
    ]
}
