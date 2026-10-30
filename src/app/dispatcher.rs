//! src/app/dispatcher.rs - ModeState + Dispatcher.

use frontier::core::bus::Bus;
use frontier::core::dispatcher::{Dispatcher, DomainReceivers};
use frontier::core::mode_switch::{Mode, ModeState};

pub struct DispatcherHandles {
    pub mode: ModeState,
    pub dispatcher: Dispatcher,
    pub domain_receivers: DomainReceivers,
}

pub fn init(bus: Bus) -> DispatcherHandles {
    let mode = ModeState::with_mode(Mode::All);
    let (dispatcher, domain_receivers) = Dispatcher::new(mode.clone(), bus, 256);
    DispatcherHandles { mode, dispatcher, domain_receivers }
}

