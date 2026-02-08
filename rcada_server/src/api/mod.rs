pub mod health;
pub mod tags;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/api/v1")
        .service(health::scope())
        .service(tags::scope())
}
