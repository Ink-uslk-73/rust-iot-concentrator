//! src/protocols/soft/thread.rs - Thread (OpenThread RCP).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct ThreadAdapter;

impl ThreadAdapter {
    pub fn new() -> Self { Self }
}

impl Default for ThreadAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for ThreadAdapter {
    fn name(&self) -> &'static str { "thread" }
    fn priority(&self) -> Priority { Priority::Soft }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
