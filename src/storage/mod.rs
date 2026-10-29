//! src/storage/mod.rs - storage layer (PostgreSQL + JSON fallback).

pub mod raw;
pub mod baselines;
pub mod cache;
pub mod events;
pub mod fallback;
pub mod pg;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("db: {0}")]
    Db(#[from] sqlx::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;
pub mod store_forward;
