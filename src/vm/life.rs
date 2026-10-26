//! src/vm/life.rs - LIFE domain (PLATE-C, 1000 ms).
use crate::core::bus::BusMessage;
use crate::core::dispatcher::DomainKind;
use crate::core::mode_switch::Priority;
use crate::vm::Domain;

pub struct LifeDomain {
    pub processed: u64,
}

impl Default for LifeDomain {
    fn default() -> Self {
        Self { processed: 0 }
    }
}

impl Domain for LifeDomain {
    fn kind(&self) -> DomainKind { DomainKind::Life }
    fn priority(&self) -> Priority { Priority::Life }
    fn handle(&mut self, _msg: BusMessage) { self.processed += 1; }
    fn snapshot(&self) -> Vec<u8> { self.processed.to_le_bytes().to_vec() }
    fn restore(&mut self, state: &[u8]) {
        if state.len() >= 8 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&state[..8]);
            self.processed = u64::from_le_bytes(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Domain;

    #[test]
    fn snapshot_roundtrip() {
        let mut d = LifeDomain::default();
        d.processed = 99;
        let s = d.snapshot();
        d.processed = 0;
        d.restore(&s);
        assert_eq!(d.processed, 99);
    }
}
