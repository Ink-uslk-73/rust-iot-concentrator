//! src/app/mod.rs - точка входа приложения (wiring).

pub mod bus;
pub mod config;
pub mod dispatcher;
pub mod domains;
pub mod satellite;

pub use config::{Cli, Cmd};

use tracing::info;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    info!("=== Frontier v4.0 ===");
    match cli.command {
        Cmd::Run => {
            info!("run mode - step 3/5: domains + satellite");
            let bus_h = bus::init();
            let mut disp_h = dispatcher::init(bus_h.bus.clone());
            let mut dom_h = domains::init(bus_h.bus.clone());
            domains::spawn_domains(&mut dom_h.supervisor, bus_h.bus.clone(), disp_h.domain_receivers,
                dom_h.hard_sem, dom_h.soft_sem, dom_h.life_sem);
            let _sat_h = satellite::init(bus_h.bus.clone());
            Ok(())
        }
        Cmd::Term => {
            info!("terminal mode - TBD");
            Ok(())
        }
    }
}

