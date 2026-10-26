//! Storage tests: fallback (no DB) + versioning.

use chrono::Utc;
use rust_iot_concentrator::storage::baselines::{self, Baseline};
use rust_iot_concentrator::storage::events::{self, Event, EventFilter};

#[tokio::test]
async fn baseline_save_latest_history_no_db() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let b1 = Baseline {
        device_id: "aig-01".into(),
        version: 1,
        hash: baselines::sha256_hex(b"v1"),
        captured_at: Utc::now(),
        operator: "ops".into(),
        reason: "init".into(),
        approval_ticket: Some("T-1".into()),
        payload: serde_json::json!({"critical_paths": [], "high_paths": []}),
        git_commit: None,
    };
    baselines::save(None, root, &b1).await.unwrap();

    let got = baselines::latest(None, root, "aig-01").await.unwrap().unwrap();
    assert_eq!(got.version, 1);
    assert_eq!(got.operator, "ops");

    let b2 = Baseline { version: 2, ..b1.clone() };
    baselines::save(None, root, &b2).await.unwrap();
    let hist = baselines::history(None, root, "aig-01", 10).await.unwrap();
    assert_eq!(hist.len(), 2);
    assert_eq!(hist[0].version, 2);
}

#[tokio::test]
async fn event_append_list_ack_no_db() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let e = Event {
        event_id: "evt-001".into(),
        ts: Utc::now(),
        device_id: "aig-01".into(),
        event_type: "config_drift".into(),
        severity: "critical".into(),
        technique_id: Some("T0836".into()),
        tactic: Some("Impair Process Control".into()),
        details: serde_json::json!({"path": "/x"}),
        source: "aig-rest".into(),
        acked: false,
        acked_by: None,
        acked_at: None,
    };
    events::append(None, root, &e).await.unwrap();
    let list = events::list(None, &EventFilter { limit: Some(10), ..Default::default() }).await.unwrap();
    assert_eq!(list.len(), 0); // no DB -> list returns empty
}

#[test]
fn next_version_increments() {
    let existing = vec![
        Baseline { device_id: "d".into(), version: 1, hash: "".into(), captured_at: Utc::now(), operator: "".into(), reason: "".into(), approval_ticket: None, payload: serde_json::json!({}), git_commit: None },
        Baseline { device_id: "d".into(), version: 3, hash: "".into(), captured_at: Utc::now(), operator: "".into(), reason: "".into(), approval_ticket: None, payload: serde_json::json!({}), git_commit: None },
    ];
    assert_eq!(baselines::next_version(&existing), 4);
}

#[test]
fn sha256_hex_stable() {
    assert_eq!(baselines::sha256_hex(b""), baselines::sha256_hex(b""));
    assert_ne!(baselines::sha256_hex(b"a"), baselines::sha256_hex(b"b"));
}
