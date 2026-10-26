//! src/protocols/soft/matter.rs - Matter (controller).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct MatterAdapter;

impl MatterAdapter {
    pub fn new() -> Self { Self }
}

impl Default for MatterAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for MatterAdapter {
    fn name(&self) -> &'static str { "matter" }
    fn priority(&self) -> Priority { Priority::Soft }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
