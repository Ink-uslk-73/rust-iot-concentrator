//! src/automation/mod.rs - AutomationRunner (P3.50, рутина).

pub mod tasks;

use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use tokio::sync::broadcast;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::api::chat::ChatMsg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomationState {
    Idle = 0,
    Running = 1,
    Paused = 2,
    Rollback = 3,
}

pub struct AutomationRunner {
    state: Arc<AtomicU8>,
    chat_tx: Option<broadcast::Sender<ChatMsg>>,
    sched: JobScheduler,
}

impl AutomationRunner {
    pub async fn new(chat_tx: Option<broadcast::Sender<ChatMsg>>) -> Result<Self, String> {
        let sched = JobScheduler::new().await.map_err(|e| e.to_string())?;
        Ok(Self { state: Arc::new(AtomicU8::new(AutomationState::Idle as u8)), chat_tx, sched })
    }

    pub fn state(&self) -> AutomationState {
        match self.state.load(Ordering::SeqCst) {
            0 => AutomationState::Idle,
            1 => AutomationState::Running,
            2 => AutomationState::Paused,
            _ => AutomationState::Rollback,
        }
    }

    pub fn set_state(&self, s: AutomationState) {
        self.state.store(s as u8, Ordering::SeqCst);
    }

    fn publish(&self, text: String, tab: &str) {
        if let Some(tx) = &self.chat_tx {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            let _ = tx.send(ChatMsg {
                text,
                tab: tab.to_string(),
                role: "automation".into(),
                ts: now,
            });
        }
    }

    pub async fn add_daily_report(&mut self, cron: &str) -> Result<(), String> {
        let tx = self.chat_tx.clone();
        let job = Job::new_async(cron, move |_uuid, _l| {
            let tx = tx.clone();
            Box::pin(async move {
                if let Some(tx) = &tx {
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
            })
        }).map_err(|e| e.to_string())?;
        self.sched.add(job).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn start(&self) -> Result<(), String> {
        self.sched.start().await.map_err(|e| e.to_string())?;
        self.set_state(AutomationState::Running);
        self.publish("automation started".into(), "engineering");
        Ok(())
    }

    pub async fn shutdown(&mut self) {
        let _ = self.sched.shutdown().await;
        self.set_state(AutomationState::Idle);
    }
}
