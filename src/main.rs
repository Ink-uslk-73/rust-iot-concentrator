//! src/main.rs - entrypoint: CLI + app::run.

use clap::Parser;

use frontier::app::{run, Cli};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    frontier::app::config::init_logging();
    run(cli).await
}

