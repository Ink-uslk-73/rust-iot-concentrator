//! src/main.rs - entrypoint: CLI, wiring, supervisor, satellite, cache.

use std::sync::{Arc, RwLock};
use std::time::Duration;
use clap::{Parser, Subcommand};
use tracing::{info, warn};

use rust_iot_concentrator::api::{router, AppState};
use rust_iot_concentrator::cache_tasks::spawn_cache_tasks;
use rust_iot_concentrator::config::ConcentratorConfig;
use rust_iot_concentrator::core::bus::{Bus, BusMessage, BusPayload};
use rust_iot_concentrator::core::dispatcher::Dispatcher;
use rust_iot_concentrator::core::mode_switch::{Mode, ModeState, Priority};
use rust_iot_concentrator::gateway::GatewayConfig;
use rust_iot_concentrator::metrics::OverheadMetrics;
use rust_iot_concentrator::protocols;
use rust_iot_concentrator::satellite::heartbeat::spawn_heartbeat;
use rust_iot_concentrator::satellite::watchdog::SatelliteWatchdog;
use rust_iot_concentrator::satellite::SatelliteRegistry;
use rust_iot_concentrator::storage::cache::HotCache;
use rust_iot_concentrator::storage::pg;
use rust_iot_concentrator::storage::raw::RawStore;
use rust_iot_concentrator::supervisor::Supervisor;
use rust_iot_concentrator::vm::hard::HardDomain;
use rust_iot_concentrator::vm::life::LifeDomain;
use rust_iot_concentrator::vm::run_domain_with_sem;
use rust_iot_concentrator::vm::soft::SoftDomain;
use rust_iot_concentrator::vm::state::VmSemaphore;

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
    // 1. Config
    let concentrator_cfg = ConcentratorConfig::load(&cli.config)
        .map_err(|e| anyhow::anyhow!("config: {}", e))?;
    let (mc, mr) = concentrator_cfg.modbus.current_paths();
    info!(mode = %concentrator_cfg.modbus.mode, config = %mc, registers = %mr, "modbus paths");

    // 2. Bus + metrics + registry + self-telemetry
    let metrics = Arc::new(OverheadMetrics::new());
    let registry = Arc::new(rust_iot_concentrator::metrics::registry::Registry::new());
    let mut self_tel = rust_iot_concentrator::metrics::self_telemetry::SelfTelemetry::new();
    let registry_bg = registry.clone();
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(2));
        loop { tick.tick().await; self_tel.publish(&registry_bg); }
    });

    let (bus, bus_rx) = Bus::new(256, 4096, 128, 128, metrics.clone());
    info!("bus initialized");

    // 3. Mode + dispatcher
    let mode = ModeState::with_mode(Mode::All);
    info!(mode = mode.get().name(), "initial mode");
    let (signals_rx, telemetry_rx, status_rx, diag_rx) = bus_rx.split();
    let (dispatcher, domain_receivers) = Dispatcher::new(mode.clone(), bus.clone(), 256);

    // 4. Satellite registry + watchdog
    let satellite_registry = Arc::new(SatelliteRegistry::new());
    let sat_wd = SatelliteWatchdog::new(satellite_registry.clone(), bus.clone());
    tokio::spawn(sat_wd.run());

    // 5. Supervisor + semaphores
    let mut supervisor = Supervisor::new().with_bus(bus.clone());
    let hard_sem = VmSemaphore::new();
    let soft_sem = VmSemaphore::new();
    let life_sem = VmSemaphore::new();
    supervisor.register("hard", hard_sem.clone());
    supervisor.register("soft", soft_sem.clone());
    supervisor.register("life", life_sem.clone());

    let (dummy_tx_soft, dummy_rx_soft) = tokio::sync::mpsc::channel(1);
    let (dummy_tx_life, dummy_rx_life) = tokio::sync::mpsc::channel(1);
    std::mem::forget(dummy_tx_soft);
    std::mem::forget(dummy_tx_life);

    // 6. Domains
    let hard_bus = bus.clone();
    let hard_sem2 = hard_sem.clone();
    supervisor.spawn(async move {
        run_domain_with_sem(HardDomain::default(), domain_receivers.hard,
            domain_receivers.commands, hard_bus, Duration::from_millis(10), hard_sem2).await;
    });
    let soft_bus = bus.clone();
    let soft_sem2 = soft_sem.clone();
    supervisor.spawn(async move {
        run_domain_with_sem(SoftDomain::default(), domain_receivers.soft,
            dummy_rx_soft, soft_bus, Duration::from_millis(100), soft_sem2).await;
    });
    let life_bus = bus.clone();
    let life_sem2 = life_sem.clone();
    supervisor.spawn(async move {
        run_domain_with_sem(LifeDomain::default(), domain_receivers.life,
            dummy_rx_life, life_bus, Duration::from_millis(1000), life_sem2).await;
    });

    // 7. Dispatcher loop
    supervisor.spawn(async move {
        if let Err(e) = dispatcher.run(telemetry_rx, signals_rx, status_rx, diag_rx).await {
            warn!("dispatcher error: {}", e);
        }
    });
    info!("domains + dispatcher started");

    // 8. Protocols + satellite heartbeats
    let cfg_arc = Arc::new(RwLock::new(concentrator_cfg));
    let satellite_registry_hb = satellite_registry.clone();
    for a in protocols::create_all(&cfg_arc.read().unwrap()) {
        info!(protocol = a.name(), priority = ?a.priority(), "adapter registered");
        if let Some(sat_name) = a.heartbeat_name() {
            spawn_heartbeat(satellite_registry_hb.clone(), sat_name, Arc::clone(&a), registry.clone(), Duration::from_secs(2), 3000);
            info!(satellite = sat_name, "heartbeat spawned");
        }
    }

    // 9. Gateway config
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

    // 10. PostgreSQL
    let pool = match pg::connect(&cli.database_url).await {
        Ok(p) => { info!("postgresql connected"); Some(p) }
        Err(e) => { warn!(error = %e, "postgresql unavailable; file-fallback mode"); None }
    };

    // 11. Cache (Redis optional) + cache_tasks (flush + sync)
    let cache: Option<Arc<HotCache>> = {
        let cfg = cfg_arc.read().unwrap();
        if cfg.cache.enabled {
            match (cfg.cache.url.as_deref(), cfg.cache.ttl_sec) {
                (Some(url), Some(ttl)) => match HotCache::new(url, ttl) {
                    Ok(c) => { info!(url, ttl, "redis cache enabled"); Some(Arc::new(c)) }
                    Err(e) => { warn!("redis init: {}", e); None }
                },
                _ => { warn!("cache.enabled=true but url/ttl missing"); None }
            }
        } else { None }
    };

    let raw_path = cfg_arc.read().unwrap().cache.raw_path.clone()
        .unwrap_or_else(|| "storage/raw/telemetry.cbor".into());
    let raw = Arc::new(RawStore::new(&raw_path));
    let flush_sec = cfg_arc.read().unwrap().cache.flush_interval_sec.unwrap_or(60);
    spawn_cache_tasks(
        cache.clone(),
        raw,
        pool.clone(),
        bus.clone(),
        Duration::from_secs(flush_sec),
        Duration::from_secs(60),
    );

    // 12. API
    let api_addr = cfg_arc.read().unwrap().api.addr.clone()
        .unwrap_or_else(|| cli.api_addr.clone());

    let state: Arc<AppState> = Arc::new(AppState {
        pool: pool.clone(),
        root: std::path::PathBuf::from("."),
        metrics: metrics.clone(),
        registry: registry.clone(),
        cache: cache.clone(),
        mode: mode.clone(),
        bus: bus.clone(),
        config: cfg_arc.clone(),
        satellites: satellite_registry.clone(),
    });
    let app = router(state);
    let listener = tokio::net::TcpListener::bind(&api_addr).await?;
    info!(addr = %api_addr, "api server listening");
    let api_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            warn!("api server error: {}", e);
        }
    });

    // 13. Supervisor run + Ctrl+C
    let sup_handle = tokio::spawn(supervisor.run());
    tokio::signal::ctrl_c().await?;
    info!("shutdown signal received");
    let _ = bus.send(BusMessage::new("main", Priority::Hard, BusPayload::Shutdown)).await;
    tokio::time::sleep(Duration::from_millis(100)).await;

    api_handle.abort();
    sup_handle.abort();
    info!("shutdown complete");
    Ok(())
}
