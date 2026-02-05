use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use tracing_actix_web::TracingLogger;

mod api;
mod container;
mod tag_storage;

use container::Container;
use tag_storage::adapter::inmemory::TagStorage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting RCADA server");

    let container = Container::new(TagStorage::new());

    tracing::info!("Initialized tag storage repository");

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
