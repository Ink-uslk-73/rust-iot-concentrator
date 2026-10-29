//! src/satellite/satellite_id.rs - идентификатор сателлита (PLATE-A/B/C).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SatelliteId { A, B, C }

impl SatelliteId {
    pub fn as_str(self) -> &'static str {
        match self { Self::A => "A", Self::B => "B", Self::C => "C" }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s { "A" => Some(Self::A), "B" => Some(Self::B), "C" => Some(Self::C), _ => None }
    }
}

impl std::fmt::Display for SatelliteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        for id in [SatelliteId::A, SatelliteId::B, SatelliteId::C] {
            assert_eq!(SatelliteId::from_str(id.as_str()), Some(id));
        }
    }

    #[test]
    fn unknown() { assert_eq!(SatelliteId::from_str("X"), None); }
}

