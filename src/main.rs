//! src/main.rs - entrypoint: CLI, wiring, shutdown.

use std::sync::{Arc, RwLock};
use clap::{Parser, Subcommand};
use tracing::{info, warn};

use rust_iot_concentrator::api::{router, AppState};
use rust_iot_concentrator::config::ConcentratorConfig;
use rust_iot_concentrator::core::bus::{Bus, BusMessage, BusPayload};
use rust_iot_concentrator::core::dispatcher::Dispatcher;
use rust_iot_concentrator::core::mode_switch::{Mode, ModeState, Priority};
use rust_iot_concentrator::gateway::GatewayConfig;
use rust_iot_concentrator::metrics::OverheadMetrics;
use rust_iot_concentrator::protocols;
use rust_iot_concentrator::storage::pg;
use rust_iot_concentrator::vm::hard::HardDomain;
use rust_iot_concentrator::vm::life::LifeDomain;
use rust_iot_concentrator::vm::run_domain;
use rust_iot_concentrator::vm::soft::SoftDomain;

#[derive(Parser)]
#[command(name = "rust-iot-concentrator", version, about = "Telecom concentrator v4.0")]
struct Cli {
    #[arg(long, default_value = "config/concentrator.toml")]
    config: String,
    #[arg(long, default_value = "postgres://concentrator@localhost:5432/concentrator")]
    database_url: String,
    #[arg(long, default_value = "0.0.0.0:8443")]
    api_addr: String,
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd { Run, Term }

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
        Cmd::Run => run(cli).await?,
        Cmd::Term => info!("terminal mode (standalone) - not implemented yet"),
    }
    Ok(())
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let concentrator_cfg = ConcentratorConfig::load(&cli.config)
        .map_err(|e| anyhow::anyhow!("config: {}", e))?;
    let (mc, mr) = concentrator_cfg.modbus.current_paths();
    info!(mode = %concentrator_cfg.modbus.mode, config = %mc, registers = %mr, "modbus paths");
let metrics = Arc::new(OverheadMetrics::new());
let registry = Arc::new(rust_iot_concentrator::metrics::registry::Registry::new());
let mut self_tel = rust_iot_concentrator::metrics::self_telemetry::SelfTelemetry::new();
let registry_bg = registry.clone();
tokio::spawn(async move {
    let mut tick = tokio::time::interval(std::time::Duration::from_secs(2));
    loop {
        tick.tick().await;
        self_tel.publish(&registry_bg);
    }
});
let (bus, bus_rx) = Bus::new(256, 4096, 128, 128, metrics.clone());
    info!("bus initialized");

    let mode = ModeState::with_mode(Mode::All);
    info!(mode = mode.get().name(), "initial mode");

    let (signals_rx, telemetry_rx, status_rx, diag_rx) = bus_rx.split();
    let (dispatcher, domain_receivers) = Dispatcher::new(mode.clone(), bus.clone(), 256);

    let (dummy_tx_soft, dummy_rx_soft) = tokio::sync::mpsc::channel(1);
    let (dummy_tx_life, dummy_rx_life) = tokio::sync::mpsc::channel(1);
    std::mem::forget(dummy_tx_soft);
    std::mem::forget(dummy_tx_life);

    let hard_handle = tokio::spawn(run_domain(
        HardDomain::default(), domain_receivers.hard, domain_receivers.commands,
        bus.clone(), std::time::Duration::from_millis(10),
    ));
    let soft_handle = tokio::spawn(run_domain(
        SoftDomain::default(), domain_receivers.soft, dummy_rx_soft,
        bus.clone(), std::time::Duration::from_millis(100),
    ));
    let life_handle = tokio::spawn(run_domain(
        LifeDomain::default(), domain_receivers.life, dummy_rx_life,
        bus.clone(), std::time::Duration::from_millis(1000),
    ));
    let dispatcher_handle = tokio::spawn(async move {
        if let Err(e) = dispatcher.run(telemetry_rx, signals_rx, status_rx, diag_rx).await {
            warn!("dispatcher error: {}", e);
        }
    });
    info!("domains + dispatcher started");

    let cfg_arc = Arc::new(RwLock::new(concentrator_cfg));
    for a in protocols::create_all(&cfg_arc.read().unwrap()) {
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

    let pool = match pg::connect(&cli.database_url).await {
        Ok(p) => { info!("postgresql connected"); Some(p) }
        Err(e) => { warn!(error = %e, "postgresql unavailable; running in file-fallback mode"); None }
    };

    let state: Arc<AppState> = Arc::new(AppState {
    pool: pool.clone(),
    root: std::path::PathBuf::from("."),
    metrics: metrics.clone(),
    registry: registry.clone(),   // <-- добавить
    mode: mode.clone(),
    bus: bus.clone(),
    config: cfg_arc.clone(),
    });
    let app = router(state);
    let listener = tokio::net::TcpListener::bind(&cli.api_addr).await?;
    info!(addr = %cli.api_addr, "api server listening");
    let api_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            warn!("api server error: {}", e);
        }
    });

    let sync_handle = if let Some(pool) = pool.clone() {
        let pool = pool.clone();
        Some(tokio::spawn(async move {
            let mut tick = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                tick.tick().await;
                let root = std::path::Path::new(".");
                match rust_iot_concentrator::storage::fallback::sync_baselines(&pool, root).await {
                    Ok(n) if n > 0 => info!(count = n, "synced baselines"),
                    Ok(_) => {}
                    Err(e) => warn!("sync_baselines: {}", e),
                }
                match rust_iot_concentrator::storage::fallback::sync_events(&pool, root).await {
                    Ok(n) if n > 0 => info!(count = n, "synced events"),
                    Ok(_) => {}
                    Err(e) => warn!("sync_events: {}", e),
                }
            }
        }))
    } else { None };

    tokio::signal::ctrl_c().await?;
    info!("shutdown signal received");
    let _ = bus.send(BusMessage::new("main", Priority::Hard, BusPayload::Shutdown)).await;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    api_handle.abort();
    if let Some(h) = sync_handle { h.abort(); }
    dispatcher_handle.abort();
    hard_handle.abort();
    soft_handle.abort();
    life_handle.abort();
    info!("shutdown complete");
    Ok(())
}
