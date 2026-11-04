//! src/api/chat.rs - Chat (POST + SSE, P3.43).

use axum::{extract::{State, Extension}, http::StatusCode, Json, response::sse::{Event as SseEvent, Sse}};
use tokio_stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;

use super::SharedState;
use super::rbac::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMsg {
    pub text: String,
    pub tab: String,
    pub role: String,
    pub ts: u64,
}

#[derive(Debug, Deserialize)]
pub struct PostChat {
    pub text: String,
    #[serde(default = "default_tab")]
    pub tab: String,
}

fn default_tab() -> String { "engineering".into() }

pub async fn post_chat(
    State(s): State<SharedState>,
    Extension(role): Extension<Role>,
    Json(body): Json<PostChat>,
) -> Result<Json<ChatMsg>, StatusCode> {
    let Some(tx) = &s.chat_tx else { return Err(StatusCode::SERVICE_UNAVAILABLE); };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let msg = ChatMsg { text: body.text, tab: body.tab, role: format!("{:?}", role), ts: now };
    let _ = tx.send(msg.clone());
    Ok(Json(msg))
}

pub async fn stream_chat(
    State(s): State<SharedState>,
) -> Sse<futures_util::stream::BoxStream<'static, Result<SseEvent, Infallible>>> {
    use futures_util::stream::BoxStream;
    let Some(tx) = &s.chat_tx else {
        let empty: BoxStream<'static, Result<SseEvent, Infallible>> = Box::pin(futures_util::stream::empty());
        return Sse::new(empty).keep_alive(
            axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15))
        );
    };
    let rx = tx.subscribe();
    let s: BoxStream<'static, Result<SseEvent, Infallible>> = Box::pin(
        tokio_stream::wrappers::BroadcastStream::new(rx)
            .filter_map(|r| r.ok())
            .map(|msg| {
                let data = serde_json::to_string(&msg).unwrap_or_default();
                Ok(SseEvent::default().event("chat").data(data))
            }),
    );
    Sse::new(s).keep_alive(
        axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15))
    )
}
