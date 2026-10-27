//! src/gateway/ui.rs - embedded UI (rust-embed).
//!
//! Serves ui/dist/ from the binary. Build UI first: cd ui && npm run build.

use axum::{http::{header, StatusCode, Uri}, response::{IntoResponse, Response}};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "ui/dist/"]
pub struct UiAssets;

pub async fn serve(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match UiAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => {
            // SPA fallback: serve index.html for client-side routes
            match UiAssets::get("index.html") {
                Some(content) => (
                    [(header::CONTENT_TYPE, "text/html")],
                    content.data,
                )
                    .into_response(),
                None => (StatusCode::NOT_FOUND, "UI not built. Run: cd ui && npm run build").into_response(),
            }
        }
    }
}
