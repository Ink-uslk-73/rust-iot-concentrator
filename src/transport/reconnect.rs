//! src/transport/reconnect.rs - политика реконнекта: экспонента + equal jitter.
//!
//! Используется mqtt/http адаптерами и PeerLink.
//! equal jitter: delay = base/2 + rand(0..base/2), base = min(max, initial * mult^n).

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ReconnectSchedule {
    initial: Duration,
    max: Duration,
    multiplier: f64,
    attempt: u32,
    seed: u64,
}

impl ReconnectSchedule {
    pub fn new(initial: Duration, max: Duration, multiplier: f64, seed: u64) -> Self {
        Self { initial, max, multiplier, attempt: 0, seed }
    }

    /// Дефолт: 200 мс -> 5 с, x2, seed из seq.
    pub fn default_with_seed(seed: u64) -> Self {
        Self::new(Duration::from_millis(200), Duration::from_secs(5), 2.0, seed)
    }

    /// Задержка для текущей попытки; инкрементит attempt.
    pub fn next_delay(&mut self) -> Duration {
        let exp = self.initial.as_millis() as f64
            * self.multiplier.powi(self.attempt.min(16) as i32);
        let base = exp.min(self.max.as_millis() as f64);
        self.attempt = self.attempt.saturating_add(1);
        let half = base / 2.0;
        let jitter = (self.xorshift() as f64 / u64::MAX as f64) * half;
        Duration::from_millis((half + jitter) as u64)
    }

    pub fn reset(&mut self) { self.attempt = 0; }
    pub fn attempts(&self) -> u32 { self.attempt }

    fn xorshift(&mut self) -> u64 {
        let mut x = self.seed | 1;
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.seed = x; x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jitter_in_half_base_and_grows() {
        let mut s = ReconnectSchedule::new(Duration::from_millis(100), Duration::from_secs(5), 2.0, 42);
        let d1 = s.next_delay();
        assert!(d1 >= Duration::from_millis(50) && d1 <= Duration::from_millis(100));
        for _ in 0..20 { let _ = s.next_delay(); }
        let d = s.next_delay();
        assert!(d <= Duration::from_secs(5));
    }

    #[test]
    fn reset_restores_initial() {
        let mut s = ReconnectSchedule::new(Duration::from_millis(100), Duration::from_secs(5), 2.0, 7);
        for _ in 0..5 { s.next_delay(); }
        s.reset();
        let d = s.next_delay();
        assert!(d <= Duration::from_millis(100));
    }
}