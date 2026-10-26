//! src/protocols/life/http.rs - HTTP (axum).
use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct HttpAdapter;

impl HttpAdapter {
    pub fn new() -> Self { Self }
}

impl Default for HttpAdapter {
    fn default() -> Self { Self::new() }
}

impl ProtocolAdapter for HttpAdapter {
    fn name(&self) -> &'static str { "http" }
    fn priority(&self) -> Priority { Priority::Life }
    fn start(&self, _publish: PublishFn) -> Result<(), String> { Ok(()) }
}
