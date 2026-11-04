//! src/automation/tasks/session.rs - сессии (P3.50).

use tokio::sync::broadcast;
use crate::api::chat::ChatMsg;

pub fn publish_session(chat_tx: &Option<broadcast::Sender<ChatMsg>>, text: &str) {
    if let Some(tx) = chat_tx {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let _ = tx.send(ChatMsg {
            text: text.to_string(),
            tab: "engineering".into(),
            role: "automation".into(),
            ts: now,
        });
    }
}

pub fn open_session(chat_tx: &Option<broadcast::Sender<ChatMsg>>) {
    publish_session(chat_tx, "session opened");
}

pub fn close_session(chat_tx: &Option<broadcast::Sender<ChatMsg>>) {
    publish_session(chat_tx, "session closed");
}
