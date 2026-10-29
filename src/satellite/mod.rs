//! src/satellite/mod.rs - Satellite heartbeat (PLATE-A/B/C + AIG-502).

pub mod heartbeat;
pub mod watchdog;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SatelliteState { Unknown, Running, Stopped }

impl SatelliteState {
    pub fn as_str(self) -> &'static str {
        match self {
            SatelliteState::Unknown => "unknown",
            SatelliteState::Running => "running",
            SatelliteState::Stopped => "stopped",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteStatus {
    pub name: String,
    pub state: String,
    pub last_heartbeat_ms: u64,
}

struct SatelliteEntry {
    last_heartbeat: Instant,
    state: SatelliteState,
    breach_count: u32,
}

pub struct SatelliteRegistry {
    satellites: Mutex<HashMap<String, SatelliteEntry>>,
}

impl SatelliteRegistry {
    pub fn new() -> Self {
        Self { satellites: Mutex::new(HashMap::new()) }
    }

    pub fn heartbeat(&self, name: &str) {
        let mut g = self.satellites.lock().unwrap();
        let e = g.entry(name.to_string()).or_insert(SatelliteEntry {
            last_heartbeat: Instant::now(),
            state: SatelliteState::Running,
            breach_count: 0,
        });
        e.last_heartbeat = Instant::now();
        e.breach_count = 0;
        e.state = SatelliteState::Running;
    }

    pub fn check(&self, timeout: Duration, threshold: u32) -> Vec<SatelliteStatus> {
        let now = Instant::now();
        let mut g = self.satellites.lock().unwrap();
        let mut out = Vec::new();
        for (name, e) in g.iter_mut() {
            let elapsed = now.duration_since(e.last_heartbeat);
            if elapsed > timeout {
                e.breach_count += 1;
                if e.breach_count >= threshold {
                    e.state = SatelliteState::Stopped;
                }
            } else {
                e.state = SatelliteState::Running;
                e.breach_count = 0;
            }
            out.push(SatelliteStatus {
                name: name.clone(),
                state: e.state.as_str().to_string(),
                last_heartbeat_ms: elapsed.as_millis() as u64,
            });
        }
        out
    }
}

impl Default for SatelliteRegistry {
    fn default() -> Self { Self::new() }
}
pub mod satellite_id;
pub mod peer_agent;
