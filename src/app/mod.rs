//! src/app/mod.rs - точка входа приложения (wiring).

pub mod api;
pub mod bus;
pub mod config;
pub mod dispatcher;
pub mod domains;
pub mod satellite;
pub mod storage;

pub use config::{Cli, Cmd};

use std::sync::Arc;
use std::sync::RwLock;

use tracing::info;

use crate::core::bus::{BusMessage, BusPayload};
use crate::core::mode_switch::Priority;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    info!("=== Frontier v4.0 ===");
    match cli.command {
        Cmd::Run => {
            info!("run mode");

            let bus_h = bus::init();
            let disp_h = dispatcher::init(bus_h.bus.clone());
            let mut dom_h = domains::init(bus_h.bus.clone());
            let sat_h = satellite::init(bus_h.bus.clone());

            let cfg = Arc::new(RwLock::new(
                crate::config::ConcentratorConfig::load(&cli.config)
                    .map_err(|e| anyhow::anyhow!("config: {}", e))?,
            ));

            let st_h = storage::init(&cli.database_url, cfg.clone()).await;
            storage::spawn_tasks(st_h.cache.clone(), st_h.raw.clone(), None, bus_h.bus.clone(), cfg.clone());

            domains::spawn_domains(
                &mut dom_h.supervisor,
                bus_h.bus.clone(),
                disp_h.domain_receivers,
                dom_h.hard_sem,
                dom_h.soft_sem,
                dom_h.life_sem,
            );

            {
                let mut dispatcher = disp_h.dispatcher;
                let (signals_rx, telemetry_rx, status_rx, diag_rx) = bus_h.bus_rx.split();
                dom_h.supervisor.spawn(async move {
                    if let Err(e) = dispatcher.run(telemetry_rx, signals_rx, status_rx, diag_rx).await {
                        tracing::warn!("dispatcher error: {}", e);
                    }
                });
            }

            satellite::spawn_heartbeats(sat_h.registry.clone(), bus_h.registry.clone(), cfg.clone());

            let state = api::build_state(
                None,
                bus_h.metrics.clone(),
                bus_h.registry.clone(),
                st_h.cache.clone(),
                disp_h.mode.clone(),
                bus_h.bus.clone(),
                cfg.clone(),
                sat_h.registry.clone(),
            );
            let api_handle = api::serve(&cli.api_addr, state).await?;

            let sup_handle = tokio::spawn(dom_h.supervisor.run());
            tokio::signal::ctrl_c().await?;
            info!("shutdown signal received");
            let _ = bus_h.bus.send(BusMessage::new("main", Priority::Hard, BusPayload::Shutdown)).await;
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            api_handle.abort();
            sup_handle.abort();
            info!("shutdown complete");
            Ok(())
        }
        Cmd::Term => {
            info!("terminal mode - TBD");
            Ok(())
        }
    }
}

