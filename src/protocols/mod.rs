//! src/protocols/mod.rs - protocol adapters.

pub mod common;
pub mod plate_a;
pub mod plate_b;
pub mod plate_c;
pub mod life;
pub mod soft;

use std::sync::Arc;
use crate::config::ConcentratorConfig;
use crate::core::bus::{Bus, BusMessage, BusPayload};
use crate::core::mode_switch::Priority;
use crate::metrics::registry::Registry;

pub use common::{ProtocolAdapter, PublishFn, make_publish};


pub fn create_all(cfg: &ConcentratorConfig) -> Vec<Arc<dyn ProtocolAdapter>> {
    let mut out: Vec<Arc<dyn ProtocolAdapter>> = Vec::new();
    out.extend(plate_a::create_all(cfg));
    out.extend(plate_b::create_all());
    out.extend(plate_c::create_all());
    out
}
