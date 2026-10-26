//! src/aig/drift.rs - compare live AIG state with baseline.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drift {
    pub path: String,
    pub baseline: Option<serde_json::Value>,
    pub current: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    pub device_id: String,
    pub diffs: Vec<Drift>,
}

impl DriftReport {
    pub fn is_empty(&self) -> bool {
        self.diffs.is_empty()
    }
}

/// Compare baseline (path -> value) with current (path -> value).
/// `excluded` - paths that change normally (uptime, counters).
pub fn compute(
    device_id: &str,
    baseline: &HashMap<String, serde_json::Value>,
    current: &HashMap<String, serde_json::Value>,
    excluded: &[String],
) -> DriftReport {
    let mut diffs = Vec::new();

    for (path, b_val) in baseline {
        if excluded.iter().any(|e| path.starts_with(e)) {
            continue;
        }
        match current.get(path) {
            Some(c_val) if c_val == b_val => {}
            Some(c_val) => diffs.push(Drift {
                path: path.clone(),
                baseline: Some(b_val.clone()),
                current: Some(c_val.clone()),
            }),
            None => diffs.push(Drift {
                path: path.clone(),
                baseline: Some(b_val.clone()),
                current: None,
            }),
        }
    }

    // New paths in current not present in baseline
    for (path, c_val) in current {
        if excluded.iter().any(|e| path.starts_with(e)) {
            continue;
        }
        if !baseline.contains_key(path) {
            diffs.push(Drift {
                path: path.clone(),
                baseline: None,
                current: Some(c_val.clone()),
            });
        }
    }

    DriftReport { device_id: device_id.to_string(), diffs }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(items: &[(&str, serde_json::Value)]) -> HashMap<String, serde_json::Value> {
        items.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
    }

    #[test]
    fn no_drift_when_equal() {
        let b = map(&[("/system/general/hostName", serde_json::json!("aig-01"))]);
        let c = b.clone();
        let r = compute("aig-01", &b, &c, &[]);
        assert!(r.is_empty());
    }

    #[test]
    fn drift_when_changed() {
        let b = map(&[("/system/httpserver/httpsEnable", serde_json::json!(true))]);
        let c = map(&[("/system/httpserver/httpsEnable", serde_json::json!(false))]);
        let r = compute("aig-01", &b, &c, &[]);
        assert_eq!(r.diffs.len(), 1);
        assert_eq!(r.diffs[0].path, "/system/httpserver/httpsEnable");
    }

    #[test]
    fn excluded_paths_ignored() {
        let b = map(&[("/system/general/uptime", serde_json::json!(100))]);
        let c = map(&[("/system/general/uptime", serde_json::json!(200))]);
        let r = compute("aig-01", &b, &c, &["/system/general/uptime".to_string()]);
        assert!(r.is_empty());
    }

    #[test]
    fn new_path_is_drift() {
        let b = HashMap::new();
        let c = map(&[("/users/accounts/0/username", serde_json::json!("admin"))]);
        let r = compute("aig-01", &b, &c, &[]);
        assert_eq!(r.diffs.len(), 1);
        assert!(r.diffs[0].baseline.is_none());
    }
}
