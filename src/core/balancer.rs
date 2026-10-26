//! src/core/balancer.rs - per-NUMA-node degradation under load.
//!
//! Fixes vs v3.x:
//! - per-node, not sum over 128 cores.
//! - HARD never degrades: if HARD overloaded - alert (Fatal), but
//!   mode is not changed (otherwise contract is broken).
//! - Degradation LIFE -> SOFT. Recovery SOFT -> LIFE.
//! - Hysteresis: N consecutive ticks.
//! - CAS on ModeState, so manual change from tablet is not overwritten.

use std::time::Duration;
use tracing::{info, warn};

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::{Mode, ModeState, Priority};
use crate::monitoring::CpuSource;

#[derive(Debug, Clone)]
pub struct BalancerConfig {
    pub high_threshold: f64,
    pub critical_threshold: f64,
    pub recover_threshold: f64,
    pub hysteresis_ticks: u32,
    pub interval: Duration,
    pub hard_range: std::ops::Range<usize>,
    pub soft_range: std::ops::Range<usize>,
    pub life_range: std::ops::Range<usize>,
}

impl Default for BalancerConfig {
    fn default() -> Self {
        Self {
            high_threshold: 0.70,
            critical_threshold: 0.90,
            recover_threshold: 0.40,
            hysteresis_ticks: 5,
            interval: Duration::from_millis(100),
            hard_range: 0..8,
            soft_range: 8..32,
            life_range: 64..96,
        }
    }
}

pub struct Balancer {
    mode: ModeState,
    bus: Bus,
    cpu: Box<dyn CpuSource>,
    cfg: BalancerConfig,
    high_streak: u32,
    low_streak: u32,
}

impl Balancer {
    pub fn new(
        mode: ModeState,
        bus: Bus,
        cpu: Box<dyn CpuSource>,
        cfg: BalancerConfig,
    ) -> Self {
        Self {
            mode,
            bus,
            cpu,
            cfg,
            high_streak: 0,
            low_streak: 0,
        }
    }

    pub fn step(&mut self) -> Option<Mode> {
        let per_core = self.cpu.read();
        if per_core.is_empty() {
            return None;
        }

        let hard = avg_range(&per_core, &self.cfg.hard_range);
        let soft = avg_range(&per_core, &self.cfg.soft_range);
        let life = avg_range(&per_core, &self.cfg.life_range);

        if hard > self.cfg.critical_threshold {
            self.alert(
                DiagLevel::Fatal,
                format!("HARD CPU {:.0}% > {:.0}%", hard * 100.0, self.cfg.critical_threshold * 100.0),
            );
        }

        let current = self.mode.get();
        let overloaded = life > self.cfg.critical_threshold
            || soft > self.cfg.critical_threshold;

        if overloaded && current.enabled_count() > 1 {
            self.high_streak = self.high_streak.saturating_add(1);
            self.low_streak = 0;
            if self.high_streak >= self.cfg.hysteresis_ticks {
                let next = current.degrade();
                if next != current {
                    self.high_streak = 0;
                    if self.try_set(current, next) {
                        warn!(
                            from = current.name(),
                            to = next.name(),
                            hard = %format!("{:.0}%", hard * 100.0),
                            soft = %format!("{:.0}%", soft * 100.0),
                            life = %format!("{:.0}%", life * 100.0),
                            "balancer: degrade"
                        );
                        self.alert(
                            DiagLevel::Warning,
                            format!("DEGRADED {} -> {}", current.name(), next.name()),
                        );
                        return Some(next);
                    }
                }
            }
        } else if !overloaded && current != Mode::All {
            let calm = hard < self.cfg.recover_threshold
                && soft < self.cfg.recover_threshold
                && life < self.cfg.recover_threshold;
            if calm {
                self.low_streak = self.low_streak.saturating_add(1);
                self.high_streak = 0;
                if self.low_streak >= self.cfg.hysteresis_ticks {
                    let next = current.restore();
                    if next != current {
                        self.low_streak = 0;
                        if self.try_set(current, next) {
                            info!(
                                from = current.name(),
                                to = next.name(),
                                "balancer: recover"
                            );
                            self.alert(
                                DiagLevel::Info,
                                format!("RECOVERED {} -> {}", current.name(), next.name()),
                            );
                            return Some(next);
                        }
                    }
                }
            } else {
                self.low_streak = 0;
            }
        } else {
            self.high_streak = 0;
            self.low_streak = 0;
        }

        None
    }

    fn try_set(&self, expected: Mode, new: Mode) -> bool {
        match self.mode.compare_exchange(expected, new) {
            Ok(_) => true,
            Err(_actual) => false,
        }
    }

    fn alert(&self, level: DiagLevel, msg: String) {
        let _ = self.bus.diagnostics.try_send(BusMessage::new(
            "balancer",
            Priority::Hard,
            BusPayload::Diagnostic {
                level,
                component: "balancer".into(),
                message: msg,
            },
        ));
    }

    pub async fn run(mut self) {
        let mut tick = tokio::time::interval(self.cfg.interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tick.tick().await;
            self.step();
        }
    }
}

fn avg_range(v: &[f64], r: &std::ops::Range<usize>) -> f64 {
    let end = r.end.min(v.len());
    let start = r.start.min(end);
    crate::monitoring::avg(&v[start..end])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitoring::CpuSource;
    use std::sync::Mutex;

    struct MockCpu {
        values: Mutex<Vec<f64>>,
    }
    impl MockCpu {
        fn new(v: Vec<f64>) -> Self {
            Self { values: Mutex::new(v) }
        }
        fn set(&self, v: Vec<f64>) {
            *self.values.lock().unwrap() = v;
        }
    }
    impl CpuSource for MockCpu {
        fn read(&self) -> Vec<f64> {
            self.values.lock().unwrap().clone()
        }
        fn name(&self) -> &'static str {
            "mock"
        }
    }

    fn cfg() -> BalancerConfig {
        BalancerConfig {
            high_threshold: 0.70,
            critical_threshold: 0.90,
            recover_threshold: 0.40,
            hysteresis_ticks: 2,
            interval: Duration::from_millis(10),
            hard_range: 0..4,
            soft_range: 4..8,
            life_range: 8..12,
        }
    }

    fn make_with_state(v: Vec<f64>, mode: Mode) -> (Balancer, ModeState) {
        let (bus, _rx) = Bus::new(64, 64, 64, 64);
        let ms = ModeState::with_mode(mode);
        let b = Balancer::new(ms.clone(), bus, Box::new(MockCpu::new(v)), cfg());
        (b, ms)
    }

    #[test]
    fn no_change_when_normal() {
        let (mut b, ms) = make_with_state(vec![0.1; 12], Mode::All);
        for _ in 0..10 {
            assert!(b.step().is_none());
        }
        assert_eq!(ms.get(), Mode::All);
    }

    #[test]
    fn degrades_after_hysteresis() {
        let (mut b, ms) = make_with_state(vec![0.1; 4], Mode::All);
        let mut v = vec![0.1; 12];
        for x in &mut v[4..12] {
            *x = 0.95;
        }
        {
            let cpu = b.cpu.as_ref() as *const dyn CpuSource;
            let mock = unsafe { &*(cpu as *const MockCpu) };
            mock.set(v);
        }
        assert!(b.step().is_none());
        let r = b.step();
        assert_eq!(r, Some(Mode::HardSoft));
        assert_eq!(ms.get(), Mode::HardSoft);
    }

    #[test]
    fn recovers_after_hysteresis() {
        let (mut b, ms) = make_with_state(vec![0.1; 12], Mode::HardSoft);
        assert!(b.step().is_none());
        let r = b.step();
        assert_eq!(r, Some(Mode::All));
        assert_eq!(ms.get(), Mode::All);
    }

    #[test]
    fn hard_overload_alerts_but_does_not_degrade() {
        let (bus, mut rx) = Bus::new(64, 64, 64, 64);
        let ms = ModeState::with_mode(Mode::HardOnly);
        let mut v = vec![0.1; 12];
        for x in &mut v[0..4] {
            *x = 0.99;
        }
        let mut b = Balancer::new(ms.clone(), bus, Box::new(MockCpu::new(v)), cfg());

        for _ in 0..10 {
            let _ = b.step();
        }
        assert_eq!(ms.get(), Mode::HardOnly);

        let mut got_fatal = false;
        while let Ok(m) = rx.diagnostics.try_recv() {
            if let BusPayload::Diagnostic { level: DiagLevel::Fatal, .. } = m.payload {
                got_fatal = true;
            }
        }
        assert!(got_fatal, "expected Fatal alert for HARD overload");
    }

    #[test]
    fn does_not_override_manual_change() {
        let (mut b, ms) = make_with_state(vec![0.1; 12], Mode::HardSoft);
        ms.set(Mode::LifeOnly);
        for _ in 0..10 {
            let _ = b.step();
        }
        assert_eq!(ms.get(), Mode::LifeOnly);
    }

    #[test]
    fn empty_cpu_no_panic() {
        let (mut b, _ms) = make_with_state(vec![], Mode::All);
        assert!(b.step().is_none());
    }

    #[test]
    fn avg_range_clamped() {
        let v = vec![0.5; 3];
        let r = avg_range(&v, &(0..100));
        assert!((r - 0.5).abs() < 1e-9);
    }

    #[test]
    fn avg_range_start_beyond_end() {
        let v = vec![0.5; 3];
        let r = avg_range(&v, &(10..20));
        assert_eq!(r, 0.0);
    }
}
