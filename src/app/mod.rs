//! src/app/mod.rs - точка входа приложения (wiring).

pub mod bus;
pub mod config;
pub mod dispatcher;

pub use config::{Cli, Cmd};

use tracing::info;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    info!("=== Frontier v4.0 ===");
    match cli.command {
        Cmd::Run => {
            info!("run mode - step 2/5: bus + dispatcher");
            let _bus = bus::init();
            Ok(())
        }
        Cmd::Term => {
            info!("terminal mode - TBD");
            Ok(())
        }
    }
}

