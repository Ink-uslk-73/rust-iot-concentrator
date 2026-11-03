//! src/app/dispatcher.rs - ModeState + Dispatcher.

use crate::core::bus::Bus;
use crate::core::dispatcher::{Dispatcher, DomainReceivers};
use crate::core::mode_switch::{Mode, ModeState};

pub struct DispatcherHandles {
    pub mode: ModeState,
    pub dispatcher: Dispatcher,
    pub domain_receivers: DomainReceivers,
}

pub fn init(bus: Bus, telemetry_tx: Option<tokio::sync::broadcast::Sender<crate::core::bus::BusMessage>>) -> DispatcherHandles {
    let mode = ModeState::with_mode(Mode::All);
    let (dispatcher, domain_receivers) = Dispatcher::new(mode.clone(), bus, 256, telemetry_tx);
    DispatcherHandles { mode, dispatcher, domain_receivers }
}

