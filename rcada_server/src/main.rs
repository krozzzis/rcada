mod actor;
mod api;
mod repository;

use actix_web::{App, HttpServer, web};
use tracing_actix_web::TracingLogger;

use rcada_core::unit::Unit;

use crate::{actor::tag::TagRepositoryActor, repository::tag::inmemory::TagStorage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting RCADA server");

    let tag_storage = TagStorage::new();
    let (tag_repo_ref, tag_repo_handle) = ractor::Actor::spawn(
        Some("tag_repository".into()),
        TagRepositoryActor::default(),
        tag_storage,
    )
    .await
    .expect("Failed to start tag-repository actor");

    let (create_tag, mut result_rx) = crate::actor::tag::Message::create_tag(
        "temp",
        Unit::CelsiusDegree,
        rcada_core::value::DataType::Float,
    );
    tag_repo_ref
        .send_message(create_tag)
        .expect("Cannot send message to actor");
    if result_rx.recv().await.is_none() {
        tracing::error!("Cannot create tag");
    }

    {
        let tag_repo = tag_repo_ref.clone();
        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .app_data(web::Data::new(tag_repo.clone()))
                .service(api::scope())
        })
        .bind("127.0.0.1:8080")?
        .run();

        if let Err(e) = server.await {
            tracing::error!("HTTP server failed: {}", e);
        }
    }

    tracing::info!("Stopping tag repository actor");
    tag_repo_ref.stop(None);

    if let Err(e) = tag_repo_handle.await {
        tracing::error!("Tag repository actor stopped with error {:?}", e);
    } else {
        tracing::info!("Tag repository actor stopped gracefully");
    }

    Ok(())
}
