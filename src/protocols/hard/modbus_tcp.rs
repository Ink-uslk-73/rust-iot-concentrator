use std::sync::Arc;
// src/protocols/hard/modbus_tcp.rs - Modbus TCP Server (uplink, SCADA/HMI).
//
// NOT Client (tokio-modbus). Server on port 502 (slave).

use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};

pub struct ModbusTcpServer {
    port: u16,
}

impl ModbusTcpServer {
    pub fn new(port: u16) -> Self { Self { port } }
}

impl ProtocolAdapter for ModbusTcpServer {
    fn name(&self) -> &'static str { "modbus-tcp" }
    fn priority(&self) -> Priority { Priority::Hard }
    fn start(&self, _publish: PublishFn, _registry: Arc<crate::metrics::registry::Registry>) -> Result<(), String> {
        tracing::info!(port = self.port, "modbus tcp server (stub)");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn server_constructs() {
        let s = ModbusTcpServer::new(502);
        assert_eq!(s.name(), "modbus-tcp");
    }
}
