//! src/protocols/life/terminal.rs - Terminal (rustyline).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct TerminalAdapter;

impl TerminalAdapter {
    pub fn new() -> Self { Self }
}

impl Default for TerminalAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for TerminalAdapter {
    fn name(&self) -> &'static str { "terminal" }
    fn priority(&self) -> Priority { Priority::Life }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
