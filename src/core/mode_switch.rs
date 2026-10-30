//! src/core/mode_switch.rs - Mode switch (7 variants).
//!
//! Atomic storage via AtomicU8. Thread-safe.
//! Degradation: LIFE -> SOFT. HARD never degrades.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Hard,
    Soft,
    Life,
}

impl Priority {
    pub fn name(self) -> &'static str {
        match self {
            Priority::Hard => "HARD",
            Priority::Soft => "SOFT",
            Priority::Life => "LIFE",
        }
    }
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Priority::Hard),
            1 => Some(Priority::Soft),
            2 => Some(Priority::Life),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    HardOnly,
    SoftOnly,
    LifeOnly,
    HardSoft,
    HardLife,
    SoftLife,
    All,
}

impl Mode {
    pub fn name(self) -> &'static str {
        match self {
            Mode::HardOnly => "HARD_ONLY",
            Mode::SoftOnly => "SOFT_ONLY",
            Mode::LifeOnly => "LIFE_ONLY",
            Mode::HardSoft => "HARD+SOFT",
            Mode::HardLife => "HARD+LIFE",
            Mode::SoftLife => "SOFT+LIFE",
            Mode::All => "ALL",
        }
    }

    pub fn hard_enabled(self) -> bool {
        matches!(self, Mode::HardOnly | Mode::HardSoft | Mode::HardLife | Mode::All)
    }
    pub fn soft_enabled(self) -> bool {
        matches!(self, Mode::SoftOnly | Mode::HardSoft | Mode::SoftLife | Mode::All)
    }
    pub fn life_enabled(self) -> bool {
        matches!(self, Mode::LifeOnly | Mode::HardLife | Mode::SoftLife | Mode::All)
    }

    pub fn enabled_count(self) -> u8 {
        u8::from(self.hard_enabled())
            + u8::from(self.soft_enabled())
            + u8::from(self.life_enabled())
    }

    pub fn degrade(self) -> Mode {
        match self {
            Mode::All => Mode::HardSoft,
            Mode::HardSoft => Mode::HardOnly,
            Mode::HardLife => Mode::HardOnly,
            Mode::SoftLife => Mode::SoftOnly,
            _ => self,
        }
    }

    pub fn restore(self) -> Mode {
        match self {
            Mode::HardOnly => Mode::HardSoft,
            Mode::HardSoft => Mode::All,
            Mode::SoftOnly => Mode::SoftLife,
            Mode::SoftLife => Mode::All,
            Mode::LifeOnly => Mode::SoftLife,
            Mode::HardLife => Mode::All,
            Mode::All => Mode::All,
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "hard_only" | "HARD_ONLY" => Some(Mode::HardOnly),
            "soft_only" | "SOFT_ONLY" => Some(Mode::SoftOnly),
            "life_only" | "LIFE_ONLY" => Some(Mode::LifeOnly),
            "hard_soft" | "HARD+SOFT" => Some(Mode::HardSoft),
            "hard_life" | "HARD+LIFE" => Some(Mode::HardLife),
            "soft_life" | "SOFT+LIFE" => Some(Mode::SoftLife),
            "all" | "ALL" => Some(Mode::All),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Mode::HardOnly => 0,
            Mode::SoftOnly => 1,
            Mode::LifeOnly => 2,
            Mode::HardSoft => 3,
            Mode::HardLife => 4,
            Mode::SoftLife => 5,
            Mode::All => 6,
        }
    }
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Mode::HardOnly,
            1 => Mode::SoftOnly,
            2 => Mode::LifeOnly,
            3 => Mode::HardSoft,
            4 => Mode::HardLife,
            5 => Mode::SoftLife,
            6 => Mode::All,
            _ => {
                tracing::error!("mode_switch: corrupt u8 = {}, fallback to HardOnly", v);
                Mode::HardOnly
            }
        }
    }
}

#[derive(Clone)]
pub struct ModeState {
    inner: Arc<AtomicU8>,
    prev: Arc<AtomicU8>,
}

impl ModeState {
    pub fn new() -> Self {
        Self { inner: Arc::new(AtomicU8::new(Mode::All.to_u8())), prev: Arc::new(AtomicU8::new(Mode::All.to_u8())) }
    }

    pub fn with_mode(mode: Mode) -> Self {
        Self { inner: Arc::new(AtomicU8::new(mode.to_u8())), prev: Arc::new(AtomicU8::new(mode.to_u8())) }
    }

    pub fn get(&self) -> Mode {
        Mode::from_u8(self.inner.load(Ordering::Acquire))
    }

    pub fn set(&self, mode: Mode) -> Mode {
        Mode::from_u8(self.inner.swap(mode.to_u8(), Ordering::AcqRel))
    }

    pub fn compare_exchange(&self, current: Mode, new: Mode) -> Result<Mode, Mode> {
        self.inner
            .compare_exchange(
                current.to_u8(),
                new.to_u8(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map(Mode::from_u8)
            .map_err(Mode::from_u8)
    }

    /// Degrade: сохранить текущий mode в prev, установить следующий.
    pub fn degrade(&self) -> Mode {
        let cur = self.get();
        let next = cur.degrade();
        if next != cur {
            self.prev.store(cur.to_u8(), Ordering::Release);
            self.inner.store(next.to_u8(), Ordering::Release);
        }
        next
    }

    /// Restore: вернуть prev mode (если он ещё актуален).
    pub fn restore(&self) -> Mode {
        let prev = Mode::from_u8(self.prev.load(Ordering::Acquire));
        let cur = self.get();
        if prev != cur && prev.enabled_count() > cur.enabled_count() {
            self.inner.store(prev.to_u8(), Ordering::Release);
        }
        self.get()
    }
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_7_modes_roundtrip_u8() {
        for m in [
            Mode::HardOnly,
            Mode::SoftOnly,
            Mode::LifeOnly,
            Mode::HardSoft,
            Mode::HardLife,
            Mode::SoftLife,
            Mode::All,
        ] {
            assert_eq!(Mode::from_u8(m.to_u8()), m);
        }
    }

    #[test]
    fn degrade_never_touches_hard() {
        for m in [
            Mode::HardOnly,
            Mode::SoftOnly,
            Mode::LifeOnly,
            Mode::HardSoft,
            Mode::HardLife,
            Mode::SoftLife,
            Mode::All,
        ] {
            let d = m.degrade();
            if m.hard_enabled() {
                assert!(d.hard_enabled());
            }
        }
    }

    #[test]
    fn degrade_order_life_then_soft() {
        assert_eq!(Mode::All.degrade(), Mode::HardSoft);
        assert_eq!(Mode::HardSoft.degrade(), Mode::HardOnly);
        assert_eq!(Mode::HardLife.degrade(), Mode::HardOnly);
        assert_eq!(Mode::SoftLife.degrade(), Mode::SoftOnly);
    }

    #[test]
    fn mode_state_get_set() {
        let s = ModeState::new();
        assert_eq!(s.get(), Mode::All);
        let prev = s.set(Mode::HardOnly);
        assert_eq!(prev, Mode::All);
        assert_eq!(s.get(), Mode::HardOnly);
    }

    #[test]
    fn mode_state_compare_exchange_ok() {
        let s = ModeState::with_mode(Mode::All);
        assert_eq!(s.compare_exchange(Mode::All, Mode::HardSoft), Ok(Mode::All));
        assert_eq!(s.get(), Mode::HardSoft);
    }

    #[test]
    fn mode_state_compare_exchange_fail() {
        let s = ModeState::with_mode(Mode::All);
        assert_eq!(s.compare_exchange(Mode::HardOnly, Mode::HardSoft), Err(Mode::All));
        assert_eq!(s.get(), Mode::All);
    }

    #[test]
    fn enabled_count() {
        assert_eq!(Mode::HardOnly.enabled_count(), 1);
        assert_eq!(Mode::HardSoft.enabled_count(), 2);
        assert_eq!(Mode::All.enabled_count(), 3);
    }
}
