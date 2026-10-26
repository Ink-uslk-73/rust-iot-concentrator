//! src/main.rs - entrypoint: CLI, wiring, shutdown.

use clap::{Parser, Subcommand};
use tracing::{info, warn};

use rust_iot_concentrator::core::bus::{Bus, BusMessage, BusPayload};
use rust_iot_concentrator::core::dispatcher::{Dispatcher, DomainKind};
use rust_iot_concentrator::core::mode_switch::{Mode, ModeState, Priority};
use rust_iot_concentrator::gateway::GatewayConfig;
use rust_iot_concentrator::protocols;
use rust_iot_concentrator::vm::hard::HardDomain;
use rust_iot_concentrator::vm::life::LifeDomain;
use rust_iot_concentrator::vm::soft::SoftDomain;
use rust_iot_concentrator::vm::run_domain;

#[derive(Parser)]
#[command(name = "rust-iot-concentrator", version, about = "Telecom concentrator v4.0")]
struct Cli {
    #[arg(long, default_value = "config/concentrator.toml")]
    config: String,
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Run,
    Term,
}

fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    init_logging();
    info!("=== Rust IoT Concentrator v4.0 ===");

    match cli.command {
        Cmd::Run => run(cli.config).await?,
        Cmd::Term => info!("terminal mode (standalone) - not implemented yet"),
    }
    Ok(())
}

async fn run(_config_path: String) -> anyhow::Result<()> {
    let (bus, bus_rx) = Bus::new(256, 4096, 128, 128);
    info!("bus initialized");

    let mode = ModeState::with_mode(Mode::All);
    info!(mode = mode.get().name(), "initial mode");

    let (signals_rx, telemetry_rx, status_rx, diag_rx) = bus_rx.split();

    let (dispatcher, domain_receivers) = Dispatcher::new(mode.clone(), bus.clone(), 256);
    info!("dispatcher created");

    // Domain receivers: dispatcher has one commands channel. Route to HARD only;
    // SOFT and LIFE get an empty dummy channel that never receives commands.
    let (dummy_cmd_tx_soft, dummy_cmd_rx_soft) = tokio::sync::mpsc::channel(1);
    let (dummy_cmd_tx_life, dummy_cmd_rx_life) = tokio::sync::mpsc::channel(1);
    // Keep senders alive by leaking them (simple for skeleton).
    std::mem::forget(dummy_cmd_tx_soft);
    std::mem::forget(dummy_cmd_tx_life);

    let hard_handle = tokio::spawn(run_domain(
        HardDomain::default(),
        domain_receivers.hard,
        domain_receivers.commands,
        bus.clone(),
        std::time::Duration::from_millis(10),
    ));
    let soft_handle = tokio::spawn(run_domain(
        SoftDomain::default(),
        domain_receivers.soft,
        dummy_cmd_rx_soft,
        bus.clone(),
        std::time::Duration::from_millis(100),
    ));
    let life_handle = tokio::spawn(run_domain(
        LifeDomain::default(),
        domain_receivers.life,
        dummy_cmd_rx_life,
        bus.clone(),
        std::time::Duration::from_millis(1000),
    ));
    info!("domains started");

    let dispatcher_handle = tokio::spawn(async move {
        if let Err(e) = dispatcher
            .run(telemetry_rx, signals_rx, status_rx, diag_rx)
            .await
        {
            warn!("dispatcher error: {}", e);
        }
    });

    // Protocols: just list them, do not spawn (start takes &self, Box is not Clone).
    let adapters = protocols::create_all();
    for a in &adapters {
        info!(protocol = a.name(), priority = ?a.priority(), "adapter registered");
    }

    match GatewayConfig::load("config/gateway.toml") {
        Ok(cfg) => {
            if let Err(e) = cfg.validate() {
                warn!("gateway config invalid: {}", e);
            } else {
                info!(dashboard = %cfg.server.dashboard_addr, "gateway config loaded");
            }
        }
        Err(e) => warn!("gateway config load failed: {}", e),
    }

    tokio::signal::ctrl_c().await?;
    info!("shutdown signal received");

    let _ = bus
        .send(BusMessage::new("main", Priority::Hard, BusPayload::Shutdown))
        .await;

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    dispatcher_handle.abort();
    hard_handle.abort();
    soft_handle.abort();
    life_handle.abort();
    let _ = DomainKind::Hard;
    info!("shutdown complete");
    Ok(())
}
