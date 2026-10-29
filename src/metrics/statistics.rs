//! src/metrics/statistics.rs - aggregates for "Statistics" section.

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stats {
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub p95: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Window {
    M1,
    M5,
    H1,
    H24,
}

impl Window {
    pub fn size(self) -> usize {
        match self {
            Window::M1 => 60,
            Window::M5 => 300,
            Window::H1 => 3600,
            Window::H24 => 86_400,
        }
    }
    pub fn all() -> [Window; 4] {
        [Window::M1, Window::M5, Window::H1, Window::H24]
    }
    pub fn name(self) -> &'static str {
        match self {
            Window::M1 => "1m",
            Window::M5 => "5m",
            Window::H1 => "1h",
            Window::H24 => "24h",
        }
    }
}

type Key = (String, Window);

pub struct StatisticsStore {
    data: RwLock<HashMap<Key, VecDeque<f64>>>,
}

impl StatisticsStore {
    pub fn new() -> Self {
        Self { data: RwLock::new(HashMap::new()) }
    }

    pub fn push(&self, metric: &str, window: Window, value: f64) {
        let mut g = self.data.write().unwrap();
        let buf = g
            .entry((metric.to_string(), window))
            .or_insert_with(|| VecDeque::with_capacity(window.size()));
        if buf.len() == window.size() {
            buf.pop_front();
        }
        buf.push_back(value);
    }

    pub fn push_all_windows(&self, metric: &str, value: f64) {
        for w in Window::all() {
            self.push(metric, w, value);
        }
    }

    pub fn stats(&self, metric: &str, window: Window) -> Option<Stats> {
        let g = self.data.read().unwrap();
        let buf = g.get(&(metric.to_string(), window))?;
        if buf.is_empty() {
            return None;
        }
        Some(compute(buf))
    }

    pub fn keys(&self) -> Vec<String> {
        let g = self.data.read().unwrap();
        let mut v: Vec<String> = g.keys().map(|(m, _)| m.clone()).collect();
        v.sort();
        v.dedup();
        v
    }
}

impl Default for StatisticsStore {
    fn default() -> Self {
        Self::new()
    }
}

fn compute(buf: &VecDeque<f64>) -> Stats {
    let mut sorted: Vec<f64> = buf.iter().copied().collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    let min = sorted[0];
    let max = sorted[n - 1];
    let sum: f64 = sorted.iter().sum();
    let avg = sum / n as f64;
    let rank = ((n as f64) * 0.95).ceil() as usize;
    let idx = rank.saturating_sub(1).min(n - 1);
    let p95 = sorted[idx];
    Stats { count: n, min, max, avg, p95 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_stats() {
        let s = StatisticsStore::new();
        for v in [1.0, 2.0, 3.0, 4.0, 5.0] {
            s.push("x", Window::M1, v);
        }
        let st = s.stats("x", Window::M1).unwrap();
        assert_eq!(st.count, 5);
        assert_eq!(st.min, 1.0);
        assert_eq!(st.max, 5.0);
        assert_eq!(st.avg, 3.0);
        assert_eq!(st.p95, 5.0);
    }

    #[test]
    fn p95_nearest_rank() {
        let s = StatisticsStore::new();
        for v in 1..=100 {
            s.push("x", Window::M1, v as f64);
        }
        assert_eq!(s.stats("x", Window::M1).unwrap().p95, 97.0, "nearest-rank on window 60 (41..=100) -> 97");
    }

    #[test]
    fn window_capacity_ring() {
        let s = StatisticsStore::new();
        for v in 0..100 {
            s.push("x", Window::M1, v as f64);
        }
        let st = s.stats("x", Window::M1).unwrap();
        assert_eq!(st.count, 60);
        assert_eq!(st.min, 40.0);
        assert_eq!(st.max, 99.0);
    }
}
