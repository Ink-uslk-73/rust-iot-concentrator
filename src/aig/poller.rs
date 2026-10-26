//! src/aig/poller.rs - periodic AIG poll, drift -> event -> Bus.

use std::time::Duration;
use tracing::{info, warn};

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;
use crate::storage::baselines;
use crate::storage::events::{self, Event};

use super::attck::{self, EventKind};
use super::client::AigClient;
use super::drift;

pub struct AigPoller {
    client: AigClient,
    device_id: String,
    interval: Duration,
    bus: Bus,
    excluded_paths: Vec<String>,
}

impl AigPoller {
    pub fn new(
        base_url: impl Into<String>,
        device_id: impl Into<String>,
        interval: Duration,
        bus: Bus,
    ) -> Self {
        Self {
            client: AigClient::new(base_url),
            device_id: device_id.into(),
            interval,
            bus,
            excluded_paths: vec![
                "/system/general/lastBootTime".into(),
                "/system/general/uptime".into(),
                "/network/statistics".into(),
            ],
        }
    }

    pub async fn run(self) {
        let mut tick = tokio::time::interval(self.interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        info!(device = %self.device_id, interval = ?self.interval, "aig poller started");

        loop {
            tick.tick().await;
            if let Err(e) = self.poll_once().await {
                warn!(device = %self.device_id, "aig poll failed: {}", e);
            }
        }
    }

    async fn poll_once(&self) -> Result<(), String> {
        let state = self.client.fetch_state(&self.device_id).await
            .map_err(|e| e.to_string())?;

        // Read latest baseline from storage.
        let root = std::path::Path::new(".");
        let latest = baselines::latest(None, root, &self.device_id).await
            .map_err(|e| e.to_string())?;

        let Some(b) = latest else {
            // No baseline -> just record state as baseline candidate (not saved).
            return Ok(());
        };

        let baseline_params: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_value(b.payload.clone()).unwrap_or_default();

        let report = drift::compute(
            &self.device_id,
            &baseline_params,
            &state.parameters,
            &self.excluded_paths,
        );

        if report.is_empty() {
            return Ok(());
        }

        // Drift found -> emit event for each diff.
        for d in &report.diffs {
            let kind = classify(&d.path);
            let tag = attck::map(&kind);
            let severity = severity_for(&kind);

            let ev = Event {
                event_id: format!("{}-{}-{}", self.device_id, tag.technique_id, chrono::Utc::now().timestamp_millis()),
                ts: chrono::Utc::now(),
                device_id: self.device_id.clone(),
                event_type: format!("{:?}", kind),
                severity: severity.into(),
                technique_id: Some(tag.technique_id.clone()),
                tactic: Some(tag.tactic.clone()),
                details: serde_json::json!({
                    "path": d.path,
                    "baseline": d.baseline,
                    "current": d.current,
                }),
                source: "aig-rest".into(),
                acked: false,
                acked_by: None,
                acked_at: None,
            };

            if let Err(e) = events::append(None, root, &ev).await {
                warn!("append event: {}", e);
            }

            let _ = self.bus.diagnostics.try_send(BusMessage::new(
                "aig",
                Priority::Life,
                BusPayload::Diagnostic {
                    level: DiagLevel::Warning,
                    component: "aig".into(),
                    message: format!("drift {} -> {} ({})", d.path, tag.technique_id, tag.technique_name),
                },
            ));
        }

        Ok(())
    }
}

fn classify(path: &str) -> EventKind {
    if path.contains("httpserver") { EventKind::HttpsDisabled }
    else if path.contains("firmware") { EventKind::FirmwareChanged }
    else if path.contains("users/accounts") { EventKind::NewAccount }
    else if path.contains("password") { EventKind::PasswordChanged }
    else { EventKind::ConfigDrift }
}

fn severity_for(kind: &EventKind) -> &'static str {
    match kind {
        EventKind::HttpsDisabled | EventKind::FirmwareChanged => "critical",
        EventKind::NewAccount | EventKind::PasswordChanged | EventKind::ConfigDrift => "high",
        _ => "medium",
    }
}
