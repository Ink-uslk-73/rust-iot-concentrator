//! src/automation/tasks/report.rs - суточные отчёты (P3.50).

use tokio::sync::broadcast;
use crate::api::chat::ChatMsg;

pub fn publish_report(chat_tx: &Option<broadcast::Sender<ChatMsg>>) {
    if let Some(tx) = chat_tx {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let _ = tx.send(ChatMsg {
            text: "daily report (automation)".into(),
            tab: "audit".into(),
            role: "automation".into(),
            ts: now,
        });
    }
}
