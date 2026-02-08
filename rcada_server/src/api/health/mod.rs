pub mod handlers;
pub mod model;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/health")
        .service(handlers::health_check)
}
