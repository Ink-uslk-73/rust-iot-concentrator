//! src/main.rs - точка входа концентратора.
//!
//! Собирает: Bus, Registry (метрики), SatelliteRegistry (живость сателлитов),
//! Supervisor (задачи), адаптеры протоколов + heartbeat для каждого.

use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use rust_iot_concentrator::config::ConcentratorConfig;
use rust_iot_concentrator::core::bus::Bus;
use rust_iot_concentrator::metrics::registry::Registry;
use rust_iot_concentrator::protocols;
use rust_iot_concentrator::satellite::heartbeat::spawn_heartbeat;
use rust_iot_concentrator::satellite::SatelliteRegistry;
use rust_iot_concentrator::supervisor::Supervisor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  Логи 
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,rust_iot_concentrator=debug")),
        )
        .with_target(false)
        .compact()
        .init();

    info!("rust-iot-concentrator v4.0 starting");

    //  Конфигурация 
    let cfg_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/concentrator.toml".to_string());

    let cfg = match ConcentratorConfig::load(&cfg_path) {
        Ok(c) => {
            info!(path = %cfg_path, "config loaded");
            c
        }
        Err(e) => {
            error!(path = %cfg_path, err = %e, "config load failed");
            return Err(e.into());
        }
    };

    //  Bus 
    let overhead = Arc::new(rust_iot_concentrator::metrics::OverheadMetrics::new());
    let (bus, receivers) = Bus::new(256, 256, 256, 128, overhead);
    info!("bus initialized");

    //  Metrics Registry (hot-path) 
    let metrics = Arc::new(Registry::new());
    info!("metrics registry initialized");

    //  SatelliteRegistry (живость плат) 
    let satellites = Arc::new(SatelliteRegistry::new());
    info!("satellite registry initialized");

    //  Supervisor 
    let mut sup = Supervisor::new();

    //  Адаптеры протоколов 
    let adapters = protocols::create_all(&cfg);
    info!(count = adapters.len(), "protocol adapters created");

    //  Publish-функция: Bus (Hard/Soft/Life) 
    let publish = protocols::make_publish(bus.clone(), "concentrator");

    //  Запуск адаптеров + реальный heartbeat 
    // alive.<PLATE> пишется самим адаптером в start().
    // spawn_heartbeat читает alive.<PLATE> и, если свежая 
    // дергает registry.heartbeat(). Иначе  сателлит "мёртв".
    for a in &adapters {
        let name = a.name();
        let prio = a.priority();
        let heartbeat = a.heartbeat_name();

        match a.start(publish.clone(), Arc::clone(&metrics)) {
            Ok(()) => info!(adapter = name, ?prio, "adapter started"),
            Err(e) => {
                error!(adapter = name, err = %e, "adapter start failed");
                continue;
            }
        }

        if let Some(hb) = heartbeat {
            spawn_heartbeat(
                Arc::clone(&satellites),
                hb,
                Arc::clone(a),
                Arc::clone(&metrics),
                Duration::from_secs(2),   // tick
                3000,                      // timeout_ms для alive.<name>
            );
            info!(satellite = hb, "heartbeat task spawned (timeout 3000ms)");
        }
    }

    //  Раздача BusReceivers по доменам 
    // Без этого bus.send() копит в буфере: Receiver'ы должны
    // читать, иначе try_send вернёт Full и сообщения потеряются.
    // Каждый домен  обычный spawn (не restartable: у Receiver
    // уникальный ресурс, повторный вызов фабрики его не создаст).
    {
        use tracing::debug;

        let (mut rx_signals, mut rx_telemetry, mut rx_status, mut rx_diag) = receivers.split();

        sup.spawn(async move {
            while rx_signals.recv().await.is_some() { debug!("drain-signals: msg"); }
        });
        sup.spawn(async move {
            while rx_telemetry.recv().await.is_some() { debug!("drain-telemetry: msg"); }
        });
        sup.spawn(async move {
            while rx_status.recv().await.is_some() { debug!("drain-status: msg"); }
        });
        sup.spawn(async move {
            while rx_diag.recv().await.is_some() { debug!("drain-diagnostics: msg"); }
        });
    }
    //  Демо: spawn_restartable (P2.18b) 
    // Чистая фабрика + catch_unwind внутри spawn. Паника 
    // 1s backoff  повторный вызов фабрики. Для доменов
    // с уникальными ресурсами (mpsc::Receiver)  НЕ применять.
    sup.spawn_restartable("demo-restartable", || async move {
        info!("demo-restartable: tick");
        tokio::time::sleep(Duration::from_secs(30)).await;
    });

    //  Run 
    info!("supervisor run");
    sup.run().await;

    info!("shutdown");
    Ok(())
}
