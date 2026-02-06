mod api;
mod broker;
mod message;
mod tag_storage;

use actix_web::{App, HttpServer, web};
use tracing_actix_web::TracingLogger;

use rcada_core::unit::Unit;

use crate::broker::system_broker::SystemBroker;
use crate::tag_storage::inmemory::TagStorage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting RCADA server");

    let tag_storage = TagStorage::new();
    let broker = SystemBroker::new(tag_storage);

    let (create_tag, _) = tag_storage::Message::create_tag(
        "temp",
        Unit::CelsiusDegree,
        rcada_core::value::DataType::Float32,
    );
    broker.send(create_tag);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(broker.clone()))
            .service(api::scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
