pub mod models;

use actix_web::{
    HttpResponse, delete, get, post, put,
    web::{Data, Json, Path},
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    broker::system_broker::SystemBroker,
    tag_storage::inmemory::TagStorage,
    tag_storage::{self},
};

use self::models::{
    CreateTagRequest, CreateTagResponse, CreateTagResult, ListTagsResponse, TagResponse,
    UpdateValueRequest, UpdateValueResponse, UpdateValueResult,
};

use crate::tag_storage::message::{
    DeleteTagError, UpdateValueError, UpdateValueResult as CmdUpdateValueResult,
};

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/api/v1")
        .service(create_tag)
        .service(list_tags)
        .service(get_tag)
        .service(update_tag_value)
        .service(delete_tag)
}

#[post("/tags")]
#[instrument(skip(broker, req))]
pub async fn create_tag(
    broker: Data<SystemBroker<TagStorage>>,
    req: Json<CreateTagRequest>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    tracing::info!(%request_id, "Creating tag: {}", req.name);

    let (command, result) =
        tag_storage::Message::create_tag(req.name.clone(), req.unit, req.data_type);
    broker.send(command);
    let result = result
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match result {
        CreateTagResult::SuccessfullyCreated => {
            tracing::info!(%request_id, "Tag created: {}", req.name);
            Ok(HttpResponse::Created().json(CreateTagResponse {
                name: req.name.clone(),
                result: CreateTagResult::SuccessfullyCreated,
            }))
        }
        CreateTagResult::AlreadyExists => {
            tracing::warn!(%request_id, "Tag already exists: {}", req.name);
            Ok(HttpResponse::Conflict().json(CreateTagResponse {
                name: req.name.clone(),
                result: CreateTagResult::AlreadyExists,
            }))
        }
    }
}

#[get("/tags")]
#[instrument(skip(broker))]
pub async fn list_tags(broker: Data<SystemBroker<TagStorage>>) -> actix_web::Result<HttpResponse> {
    let (command, result) = tag_storage::Message::get_all_tags();
    broker.send(command);
    let tags = result
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let responses: Vec<TagResponse> = tags.into_iter().map(|t| t.into()).collect();
    tracing::info!("Listed {} tags", responses.len());
    Ok(HttpResponse::Ok().json(ListTagsResponse { tags: responses }))
}

#[get("/tags/{name}")]
#[instrument(skip(broker))]
pub async fn get_tag(
    broker: Data<SystemBroker<TagStorage>>,
    name: Path<String>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "Getting tag: {}", name_ref);

    let (command, result) = tag_storage::Message::get_tag(name_ref);
    broker.send(command);
    let result = result
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match result {
        Ok(tag) => {
            tracing::info!(%request_id, "Tag found: {}", name_ref);
            Ok(HttpResponse::Ok().json(TagResponse::from(tag)))
        }
        Err(_) => {
            tracing::warn!(%request_id, "Tag not found: {}", name_ref);
            Ok(HttpResponse::NotFound().body("Tag not found"))
        }
    }
}

#[put("/tags/{name}/value")]
#[instrument(skip(broker, req))]
pub async fn update_tag_value(
    broker: Data<SystemBroker<TagStorage>>,
    name: Path<String>,
    req: Json<UpdateValueRequest>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "Updating tag: {}", name_ref);

    let tag_value = req.0.into();
    let (command, result) = tag_storage::Message::update_tag_value(name_ref, tag_value);
    broker.send(command);
    let result = result
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match result {
        Ok(CmdUpdateValueResult::Updated) => {
            tracing::info!(%request_id, "Tag updated: {}", name_ref);
            Ok(HttpResponse::Ok().json(UpdateValueResponse {
                result: UpdateValueResult::Updated,
            }))
        }
        Ok(CmdUpdateValueResult::Ignored) => {
            tracing::debug!(%request_id, "Tag update ignored (same value): {}", name_ref);
            Ok(HttpResponse::Ok().json(UpdateValueResponse {
                result: UpdateValueResult::Ignored,
            }))
        }
        Err(UpdateValueError::TagNameNotFound) => {
            tracing::warn!(%request_id, "Tag not found for update: {}", name_ref);
            Ok(HttpResponse::NotFound().body("Tag not found"))
        }
        Err(UpdateValueError::NoneTimestampProvided) => {
            tracing::warn!(%request_id, "Timestamp required for update: {}", name_ref);
            Ok(HttpResponse::BadRequest().body("Timestamp is required after first update"))
        }
        Err(UpdateValueError::TimestamoOutOfOrder { previous }) => {
            tracing::warn!(%request_id, "Timestamp out of order for tag: {}", name_ref);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Timestamp out of order",
                "previous_timestamp": previous
            })))
        }
        Err(UpdateValueError::InvalidDataType { expected, actual }) => {
            tracing::warn!(%request_id, "Invalid data type for tag: {}", name_ref);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid data type",
                "expected": format!("{:?}", expected),
                "actual": format!("{:?}", actual)
            })))
        }
    }
}

#[delete("/tags/{name}")]
#[instrument(skip(broker))]
pub async fn delete_tag(
    broker: Data<SystemBroker<TagStorage>>,
    name: Path<String>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "Deleting tag: {}", name_ref);

    let (command, result) = tag_storage::Message::delete_tag(name_ref);
    broker.send(command);
    let result = result
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match result {
        Ok(_) => {
            tracing::info!(%request_id, "Tag deleted: {}", name_ref);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(DeleteTagError::TagNameNotFound) => {
            tracing::warn!(%request_id, "Tag not found for deletion: {}", name_ref);
            Ok(HttpResponse::NotFound().body("Tag not found"))
        }
    }
}
