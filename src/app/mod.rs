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

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    info!("=== Frontier v4.0 ===");
    match cli.command {
        Cmd::Run => {
            info!("run mode - step 4/5: storage + api");
            let bus_h = bus::init();
            let mut disp_h = dispatcher::init(bus_h.bus.clone());
            let mut dom_h = domains::init(bus_h.bus.clone());
            domains::spawn_domains(&mut dom_h.supervisor, bus_h.bus.clone(), disp_h.domain_receivers,
                dom_h.hard_sem, dom_h.soft_sem, dom_h.life_sem);
            let _sat_h = satellite::init(bus_h.bus.clone());
            let cfg = Arc::new(RwLock::new(crate::config::ConcentratorConfig::load(&cli.config)?));
            let st_h = storage::init(&cli.database_url, cfg.clone()).await;
            storage::spawn_tasks(st_h.cache.clone(), st_h.raw.clone(), None, bus_h.bus.clone(), cfg.clone());
            let state = api::build_state(None, bus_h.metrics.clone(), bus_h.registry.clone(),
                st_h.cache.clone(), disp_h.mode.clone(), bus_h.bus.clone(), cfg.clone(), _sat_h.registry.clone());
            let _api_h = api::serve(&cli.api_addr, state).await?;
            Ok(())
        }
        Cmd::Term => {
            info!("terminal mode - TBD");
            Ok(())
        }
    }
}

