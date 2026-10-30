//! src/app/config.rs - CLI, конфиг, логирование.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "frontier", version, about = "Telecom IoT concentrator v4.0")]
pub struct Cli {
    #[arg(long, default_value = "config/concentrator.toml")]
    pub config: String,
    #[arg(long, default_value = "postgres://concentrator@localhost:5432/concentrator")]
    pub database_url: String,
    #[arg(long, default_value = "127.0.0.1:1967")]
    pub api_addr: String,
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd { Run, Term }

pub fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}

