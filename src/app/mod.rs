//! src/app/mod.rs - точка входа приложения (wiring).

pub mod config;

pub use config::{Cli, Cmd};

use tracing::info;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    info!("=== Frontier v4.0 ===");
    match cli.command {
        Cmd::Run => {
            info!("run mode - TBD (wiring)");
            Ok(())
        }
        Cmd::Term => {
            info!("terminal mode - TBD");
            Ok(())
        }
    }
}

