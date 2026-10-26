//! src/vm/storage.rs - STORAGE domain (24 HDD, node1).
use crate::core::bus::BusMessage;
use crate::core::dispatcher::DomainKind;
use crate::core::mode_switch::Priority;
use crate::vm::Domain;

pub struct StorageDomain {
    pub written: u64,
}

impl Default for StorageDomain {
    fn default() -> Self {
        Self { written: 0 }
    }
}

impl Domain for StorageDomain {
    fn kind(&self) -> DomainKind {
        DomainKind::Life
    }
    fn priority(&self) -> Priority { Priority::Life }
    fn handle(&mut self, _msg: BusMessage) { self.written += 1; }
    fn snapshot(&self) -> Vec<u8> { self.written.to_le_bytes().to_vec() }
    fn restore(&mut self, state: &[u8]) {
        if state.len() >= 8 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&state[..8]);
            self.written = u64::from_le_bytes(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Domain;

    #[test]
    fn snapshot_roundtrip() {
        let mut d = StorageDomain::default();
        d.written = 123;
        let s = d.snapshot();
        d.written = 0;
        d.restore(&s);
        assert_eq!(d.written, 123);
    }
}
