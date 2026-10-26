//! src/aig/client.rs - HTTP client to AIG-502 REST API.
//!
//! AIG-502 exposes thingspro-api-v1 (universal REST) and thingspro twin
//! (Reported Properties). We read:
//! - /system/general       (hostName, firmwareVersion, softwareVersion)
//! - /system/httpserver    (httpsEnable, httpEnable, httpsPort)
//! - /network/ethernets    (ip, netmask, gateway, enableDhcp)
//! - /serials              (baudRate, parity, dataBits, stopBits)
//! - /security/certificates
//! - /users/accounts

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{AigError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AigState {
    pub device_id: String,
    pub captured_at: DateTime<Utc>,
    /// Flat map path -> value (e.g. "/system/general/hostName" -> "aig-01")
    pub parameters: HashMap<String, serde_json::Value>,
}

pub struct AigClient {
    base_url: String,
    http: reqwest::Client,
}

impl AigClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { base_url: base_url.into(), http }
    }

    /// Fetch current state from AIG. In production this calls
    /// thingspro-api-v1; here we support a simplified GET /api/config
    /// that returns the full JSON, and flatten it.
    pub async fn fetch_state(&self, device_id: &str) -> Result<AigState> {
        let url = format!("{}/api/config", self.base_url.trim_end_matches('/'));
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(AigError::Config(format!("GET {} -> {}", url, resp.status())));
        }
        let root: serde_json::Value = resp.json().await?;
        let mut parameters = HashMap::new();
        flatten("", &root, &mut parameters);
        Ok(AigState {
            device_id: device_id.to_string(),
            captured_at: Utc::now(),
            parameters,
        })
    }
}

/// Flatten nested JSON into path -> value.
pub fn flatten(prefix: &str, value: &serde_json::Value, out: &mut HashMap<String, serde_json::Value>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let path = if prefix.is_empty() {
                    format!("/{}", k)
                } else {
                    format!("{}/{}", prefix, k)
                };
                flatten(&path, v, out);
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let path = if prefix.is_empty() {
                    format!("/{}", i)
                } else {
                    format!("{}/{}", prefix, i)
                };
                flatten(&path, v, out);
            }
        }
        _ => {
            out.insert(prefix.to_string(), value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_simple() {
        let v = serde_json::json!({
            "system": { "general": { "hostName": "aig-01", "firmware": "1.4.0" } },
            "network": { "ethernets": [{ "ip": "10.0.0.10" }] }
        });
        let mut out = HashMap::new();
        flatten("", &v, &mut out);
        assert_eq!(out.get("/system/general/hostName").unwrap(), &serde_json::json!("aig-01"));
        assert_eq!(out.get("/network/ethernets/0/ip").unwrap(), &serde_json::json!("10.0.0.10"));
    }
}
