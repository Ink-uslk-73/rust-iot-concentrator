//! src/core/frontier.rs - Фронтир (термин, ADR-0008): 7 состояний, «улитка Улама».
//!
//! Пространственная природа: Wild (Dew) → Enriched → ... → Explored (Cloud).
//! Аналитическая: Raw → Enriched → Aggregated → Structured → Clustered → Patterned.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrontierState {
    /// Wild (Dew): автономные устройства, сырые данные.
    Wild,
    /// Enriched: парсинг, нормализация.
    Enriched,
    /// Aggregated: окна, метрики.
    Aggregated,
    /// Structured: схема данных.
    Structured,
    /// Clustered: DBSCAN / spectral.
    Clustered,
    /// Patterned: motifs / wavelets.
    Patterned,
    /// Explored (Cloud): освоено, HES/MDMS.
    Explored,
}

impl FrontierState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wild => "wild",
            Self::Enriched => "enriched",
            Self::Aggregated => "aggregated",
            Self::Structured => "structured",
            Self::Clustered => "clustered",
            Self::Patterned => "patterned",
            Self::Explored => "explored",
        }
    }

    /// Следующее состояние в конвейере (Raw → Patterned → Explored).
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Wild => Some(Self::Enriched),
            Self::Enriched => Some(Self::Aggregated),
            Self::Aggregated => Some(Self::Structured),
            Self::Structured => Some(Self::Clustered),
            Self::Clustered => Some(Self::Patterned),
            Self::Patterned => Some(Self::Explored),
            Self::Explored => None,
        }
    }

    /// Пространственная природа: Wild (Dew) vs Explored (Cloud).
    pub fn is_wild(self) -> bool { matches!(self, Self::Wild) }
    pub fn is_explored(self) -> bool { matches!(self, Self::Explored) }

    /// Аналитическая природа: всё, что между Wild и Explored.
    pub fn is_analytical(self) -> bool {
        !matches!(self, Self::Wild | Self::Explored)
    }
}

impl std::fmt::Display for FrontierState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Масштаб (виток «улитки Улама»): окно / частота.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scale {
    pub window_ms: u64,
    pub level: u8,
}

impl Scale {
    pub fn new(window_ms: u64, level: u8) -> Self { Self { window_ms, level } }
}

/// Фронтир: состояние + устройства + протоколы + масштаб + глубина (фрактал).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontier {
    pub state: FrontierState,
    pub devices: Vec<String>,
    pub protocols: Vec<String>,
    pub scale: Scale,
    pub depth: u8,
}

impl Frontier {
    pub fn new(state: FrontierState, scale: Scale, depth: u8) -> Self {
        Self { state, devices: Vec::new(), protocols: Vec::new(), scale, depth }
    }

    /// Переход к следующему состоянию (если есть).
    pub fn advance(&mut self) -> Option<FrontierState> {
        if let Some(next) = self.state.next() {
            self.state = next;
            Some(next)
        } else {
            None
        }
    }

    pub fn add_device(&mut self, id: impl Into<String>) { self.devices.push(id.into()); }
    pub fn add_protocol(&mut self, p: impl Into<String>) { self.protocols.push(p.into()); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_states() {
        let all = [
            FrontierState::Wild, FrontierState::Enriched, FrontierState::Aggregated,
            FrontierState::Structured, FrontierState::Clustered, FrontierState::Patterned,
            FrontierState::Explored,
        ];
        assert_eq!(all.len(), 7);
        for s in all { assert!(!s.as_str().is_empty()); }
    }

    #[test]
    fn next_chain_wild_to_explored() {
        let mut s = FrontierState::Wild;
        let mut count = 0;
        while let Some(n) = s.next() { s = n; count += 1; }
        assert_eq!(s, FrontierState::Explored);
        assert_eq!(count, 6);
    }

    #[test]
    fn wild_and_explored() {
        assert!(FrontierState::Wild.is_wild());
        assert!(!FrontierState::Wild.is_explored());
        assert!(FrontierState::Explored.is_explored());
        assert!(!FrontierState::Explored.is_wild());
    }

    #[test]
    fn analytical_states() {
        assert!(!FrontierState::Wild.is_analytical());
        assert!(!FrontierState::Explored.is_analytical());
        for s in [FrontierState::Enriched, FrontierState::Clustered, FrontierState::Patterned] {
            assert!(s.is_analytical());
        }
    }

    #[test]
    fn frontier_advance() {
        let mut f = Frontier::new(FrontierState::Wild, Scale::new(1000, 1), 0);
        assert_eq!(f.advance(), Some(FrontierState::Enriched));
        f.add_device("sensor-1");
        f.add_protocol("modbus");
        assert_eq!(f.devices.len(), 1);
        assert_eq!(f.protocols.len(), 1);
    }

    #[test]
    fn frontier_advance_to_end() {
        let mut f = Frontier::new(FrontierState::Wild, Scale::new(1000, 1), 0);
        for _ in 0..6 { let _ = f.advance(); }
        assert_eq!(f.state, FrontierState::Explored);
        assert_eq!(f.advance(), None);
    }
}

