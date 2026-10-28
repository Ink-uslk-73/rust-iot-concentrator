//! src/transport/nop.rs - No-op transport (fallback, tests, Windows).

use std::sync::Mutex;
use super::{Transport, TransportMessage};

pub struct NopTransport {
    queue: Mutex<Vec<TransportMessage>>,
}

impl NopTransport {
    pub fn new() -> Self {
        Self { queue: Mutex::new(Vec::new()) }
    }
}

impl Default for NopTransport {
    fn default() -> Self { Self::new() }
}

impl Transport for NopTransport {
    fn send(&self, msg: TransportMessage) -> Result<(), String> {
        self.queue.lock().unwrap().push(msg);
        Ok(())
    }
    fn recv(&self) -> Option<TransportMessage> {
        self.queue.lock().unwrap().pop()
    }
    fn name(&self) -> &'static str { "nop" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nop_send_recv() {
        let t = NopTransport::new();
        t.send(TransportMessage { source: "test".into(), priority: 0, payload: vec![1, 2, 3] }).unwrap();
        let got = t.recv().unwrap();
        assert_eq!(got.source, "test");
    }
}
