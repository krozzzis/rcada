pub mod handlers;
pub mod model;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/tags")
        .service(handlers::create_tag)
        .service(handlers::list_tags)
        .service(handlers::get_tag)
        .service(handlers::update_tag_value)
        .service(handlers::delete_tag)
}
