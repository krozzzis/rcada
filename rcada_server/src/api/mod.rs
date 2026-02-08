pub mod tags;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/api/v1").service(tags::scope())
}
