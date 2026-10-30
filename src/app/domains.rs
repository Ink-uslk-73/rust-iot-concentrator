//! src/app/domains.rs - Supervisor + VmSemaphore + домены (Hard/Soft/Life).

use std::time::Duration;

use crate::core::bus::Bus;
use crate::core::dispatcher::DomainReceivers;
use crate::supervisor::Supervisor;
use crate::vm::hard::HardDomain;
use crate::vm::life::LifeDomain;
use crate::vm::run_domain_with_sem;
use crate::vm::soft::SoftDomain;
use crate::vm::state::VmSemaphore;

pub struct DomainsHandles {
    pub supervisor: Supervisor,
    pub hard_sem: VmSemaphore,
    pub soft_sem: VmSemaphore,
    pub life_sem: VmSemaphore,
}

pub fn init(bus: Bus) -> DomainsHandles {
    let mut supervisor = Supervisor::new().with_bus(bus);
    let hard_sem = VmSemaphore::new();
    let soft_sem = VmSemaphore::new();
    let life_sem = VmSemaphore::new();
    supervisor.register("hard", hard_sem.clone());
    supervisor.register("soft", soft_sem.clone());
    supervisor.register("life", life_sem.clone());
    DomainsHandles { supervisor, hard_sem, soft_sem, life_sem }
}

pub fn spawn_domains(
    supervisor: &mut Supervisor,
    bus: Bus,
    domain_receivers: DomainReceivers,
    hard_sem: VmSemaphore,
    soft_sem: VmSemaphore,
    life_sem: VmSemaphore,
) {
    let (dummy_tx_soft, dummy_rx_soft) = tokio::sync::mpsc::channel(1);
    let (dummy_tx_life, dummy_rx_life) = tokio::sync::mpsc::channel(1);
    std::mem::forget(dummy_tx_soft);
    std::mem::forget(dummy_tx_life);

    let hard_bus = bus.clone();
    supervisor.spawn(async move {
        run_domain_with_sem(HardDomain::default(), domain_receivers.hard,
            domain_receivers.commands, hard_bus, Duration::from_millis(10), hard_sem).await;
    });
    let soft_bus = bus.clone();
    supervisor.spawn(async move {
        run_domain_with_sem(SoftDomain::default(), domain_receivers.soft,
            dummy_rx_soft, soft_bus, Duration::from_millis(100), soft_sem).await;
    });
    let life_bus = bus.clone();
    supervisor.spawn(async move {
        run_domain_with_sem(LifeDomain::default(), domain_receivers.life,
            dummy_rx_life, life_bus, Duration::from_millis(1000), life_sem).await;
    });
}

