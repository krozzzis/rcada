use actix_web::{HttpResponse, get};
use tracing::instrument;

use super::model::HealthResponse;

#[get("")]
#[instrument]
pub async fn health_check() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
    }))
}
