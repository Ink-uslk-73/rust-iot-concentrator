//! src/metrics/registry.rs - thread-safe metric registry.

use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Default)]
pub struct Registry {
    gauges: RwLock<HashMap<String, f64>>,
    counters: RwLock<HashMap<String, u64>>,
    labels: RwLock<HashMap<String, HashMap<String, String>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_gauge(&self, key: impl Into<String>, value: f64) {
        self.gauges.write().unwrap().insert(key.into(), value);
    }

    pub fn get_gauge(&self, key: &str) -> Option<f64> {
        self.gauges.read().unwrap().get(key).copied()
    }

    pub fn inc_counter(&self, key: impl Into<String>, by: u64) {
        let mut g = self.counters.write().unwrap();
        *g.entry(key.into()).or_insert(0) += by;
    }

    pub fn get_counter(&self, key: &str) -> u64 {
        self.counters.read().unwrap().get(key).copied().unwrap_or(0)
    }

    pub fn set_label(&self, key: impl Into<String>, name: impl Into<String>, value: impl Into<String>) {
        self.labels
            .write()
            .unwrap()
            .entry(key.into())
            .or_default()
            .insert(name.into(), value.into());
    }

    pub fn labels_of(&self, key: &str) -> HashMap<String, String> {
        self.labels.read().unwrap().get(key).cloned().unwrap_or_default()
    }

    pub fn gauges_snapshot(&self) -> HashMap<String, f64> {
        self.gauges.read().unwrap().clone()
    }

    pub fn counters_snapshot(&self) -> HashMap<String, u64> {
        self.counters.read().unwrap().clone()
    }

    pub fn gauge_count(&self) -> usize {
        self.gauges.read().unwrap().len()
    }

    pub fn clear(&self) {
        self.gauges.write().unwrap().clear();
        self.counters.write().unwrap().clear();
        self.labels.write().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauge_set_get() {
        let r = Registry::new();
        r.set_gauge("cpu.core.0", 0.5);
        assert_eq!(r.get_gauge("cpu.core.0"), Some(0.5));
        assert_eq!(r.get_gauge("missing"), None);
    }

    #[test]
    fn counter_increments() {
        let r = Registry::new();
        r.inc_counter("packets.rx", 1);
        r.inc_counter("packets.rx", 2);
        assert_eq!(r.get_counter("packets.rx"), 3);
    }

    #[test]
    fn labels() {
        let r = Registry::new();
        r.set_label("cpu.core.0", "core", "0");
        let l = r.labels_of("cpu.core.0");
        assert_eq!(l.get("core"), Some(&"0".to_string()));
    }

    #[test]
    fn clear_resets() {
        let r = Registry::new();
        r.set_gauge("a", 1.0);
        r.inc_counter("c", 5);
        r.clear();
        assert_eq!(r.gauge_count(), 0);
        assert_eq!(r.get_counter("c"), 0);
    }
}
