use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use actix_web::{App, HttpServer, web};
use tracing_actix_web::TracingLogger;

mod api;
mod bus;
mod container;
mod tag_storage;

use bus::BusDriver;
use bus::mock::MockBusDriver;
use container::Container;
use rcada_core::unit::Unit;
use tag_storage::adapter::inmemory::TagStorage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting RCADA server");

    let container = Container::new(TagStorage::new());

    tracing::info!("Initialized tag storage repository");

    let mut driver = MockBusDriver::new(container.clone(), 0.0, 100.0, Duration::from_millis(100));

    driver.create_tag("temperature".into(), Unit::CelsiusDegree);
    driver.register_tag("temperature".into(), None);

    tracing::info!("Created and registered temperature tag");

    let driver_mutex = Arc::new(Mutex::new(driver));
    let driver_start = driver_mutex.clone();

    tokio::spawn(async move {
        let mut driver = driver_start.lock().await;
        driver.start().await;
    });

    tracing::info!("Started MockBusDriver");

    let driver_poll = driver_mutex.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            let driver = driver_poll.lock().await;
            driver.poll().await;
        }
    });

    tracing::info!("Started periodic force poll");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(container.clone()))
            .service(api::scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
