//! src/metrics/snapshot.rs - snapshot for REST/WSS.

use serde::{Deserialize, Serialize};

use crate::metrics::registry::Registry;
use crate::metrics::statistics::{StatisticsStore, Stats, Window};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub key: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub ts_ms: u64,
    pub gauges: Vec<MetricValue>,
    pub counters: Vec<MetricValue>,
    pub stats: Vec<StatEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatEntry {
    pub key: String,
    pub window: String,
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub p95: f64,
}

impl From<Stats> for (usize, f64, f64, f64, f64) {
    fn from(s: Stats) -> Self {
        (s.count, s.min, s.max, s.avg, s.p95)
    }
}

pub fn build(registry: &Registry, stats: &StatisticsStore) -> Snapshot {
    let ts_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let gauges = registry
        .gauges_snapshot()
        .into_iter()
        .map(|(k, v)| MetricValue { key: k, value: v })
        .collect();
    let counters = registry
        .counters_snapshot()
        .into_iter()
        .map(|(k, v)| MetricValue { key: k, value: v as f64 })
        .collect();

    let mut stat_entries = Vec::new();
    for key in stats.keys() {
        for w in Window::all() {
            if let Some(s) = stats.stats(&key, w) {
                stat_entries.push(StatEntry {
                    key: key.clone(),
                    window: w.name().to_string(),
                    count: s.count,
                    min: s.min,
                    max: s.max,
                    avg: s.avg,
                    p95: s.p95,
                });
            }
        }
    }

    Snapshot { ts_ms, gauges, counters, stats: stat_entries }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot() {
        let r = Registry::new();
        let s = StatisticsStore::new();
        let snap = build(&r, &s);
        assert!(snap.gauges.is_empty());
        assert!(snap.stats.is_empty());
        assert!(snap.ts_ms > 0);
    }

    #[test]
    fn snapshot_with_data() {
        let r = Registry::new();
        r.set_gauge("cpu.avg", 0.5);
        let s = StatisticsStore::new();
        s.push("cpu.avg", Window::M1, 0.5);
        let snap = build(&r, &s);
        assert_eq!(snap.gauges.len(), 1);
        assert!(!snap.stats.is_empty());
    }
}
