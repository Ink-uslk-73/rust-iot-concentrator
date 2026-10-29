//! src/transport/frame.rs - кадр PeerLink (CBOR payload).

use serde::{Deserialize, Serialize};

use crate::satellite::satellite_id::SatelliteId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub seq: u64,
    pub src: SatelliteId,
    pub dst: SatelliteId,
    pub payload: Vec<u8>,
    pub ts_ms: u64,
}

impl Frame {
    pub fn new(src: SatelliteId, dst: SatelliteId, payload: Vec<u8>, ts_ms: u64) -> Self {
        Self { seq: crate::core::bus::next_seq(), src, dst, payload, ts_ms }
    }

    pub fn with_seq(seq: u64, src: SatelliteId, dst: SatelliteId, payload: Vec<u8>, ts_ms: u64) -> Self {
        Self { seq, src, dst, payload, ts_ms }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_new_has_seq() {
        let f = Frame::new(SatelliteId::A, SatelliteId::B, vec![1, 2, 3], 1000);
        assert!(f.seq > 0);
        assert_eq!(f.src, SatelliteId::A);
        assert_eq!(f.dst, SatelliteId::B);
        assert_eq!(f.payload, vec![1, 2, 3]);
    }

    #[test]
    fn frame_with_seq_keeps_seq() {
        let f = Frame::with_seq(42, SatelliteId::A, SatelliteId::C, vec![], 2000);
        assert_eq!(f.seq, 42);
    }
}

