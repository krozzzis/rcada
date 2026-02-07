pub mod models;

use actix_web::{
    HttpResponse, delete, get, post, put,
    web::{Data, Json, Path},
};
use ractor::ActorRef;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    actor,
    repository::tag::{CreateTagResult, DeleteTagError, UpdateValueError, UpdateValueResult},
};

use self::models::{
    CreateTagRequest, CreateTagResponse, ListTagsResponse, TagResponse, UpdateValueRequest,
    UpdateValueResponse,
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
#[instrument(skip(tag_repo_actor, req))]
pub async fn create_tag(
    tag_repo_actor: Data<ActorRef<actor::tag::Message>>,
    req: Json<CreateTagRequest>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    tracing::info!(%request_id, "request: (create_tag) name={}", req.name);

    let (command, mut reply) =
        actor::tag::Message::create_tag(req.name.clone(), req.unit, req.data_type);

    tag_repo_actor.send_message(command).map_err(|e| {
        tracing::error!(error = %e, "failed to send message to tag actor");
        actix_web::error::ErrorInternalServerError("Failed to send message to actor")
    })?;

    let result = reply.recv().await.ok_or_else(|| {
        tracing::error!("actor response channel closed before reply received");
        actix_web::error::ErrorInternalServerError("No response from actor")
    })?;

    match result {
        CreateTagResult::SuccessfullyCreated => {
            Ok(HttpResponse::Created().json(CreateTagResponse {
                name: req.name.clone(),
                result: CreateTagResult::SuccessfullyCreated,
            }))
        },
        CreateTagResult::AlreadyExists => Ok(HttpResponse::Conflict().json(CreateTagResponse {
            name: req.name.clone(),
            result: CreateTagResult::AlreadyExists,
        })),
    }
}

#[get("/tags")]
#[instrument(skip(tag_repo_actor))]
pub async fn list_tags(
    tag_repo_actor: Data<ActorRef<actor::tag::Message>>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    tracing::info!(%request_id, "request (list_tags)");
    let (command, mut reply) = actor::tag::Message::get_all_tags();
    tag_repo_actor
        .send_message(command)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let tags = reply.recv().await.ok_or_else(|| {
        tracing::error!("actor response channel closed before reply received");
        actix_web::error::ErrorInternalServerError("Channel closed")
    })?;

    let responses: Vec<TagResponse> = tags.into_iter().map(|t| t.into()).collect();
    Ok(HttpResponse::Ok().json(ListTagsResponse {
        tags: responses,
    }))
}

#[get("/tags/{name}")]
#[instrument(skip(tag_repo_actor))]
pub async fn get_tag(
    tag_repo_actor: Data<ActorRef<actor::tag::Message>>,
    name: Path<String>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "request: {} (get_tag)", name_ref);

    let (command, mut reply) = actor::tag::Message::get_tag(name_ref);
    tag_repo_actor
        .send_message(command)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = reply.recv().await.ok_or_else(|| {
        tracing::error!("actor response channel closed before reply received");
        actix_web::error::ErrorInternalServerError("Channel closed")
    })?;

    match result {
        Ok(tag) => Ok(HttpResponse::Ok().json(TagResponse::from(tag))),
        Err(_) => {
            tracing::warn!(%request_id, "Tag not found: {}", name_ref);
            Ok(HttpResponse::NotFound().body("Tag not found"))
        },
    }
}

#[put("/tags/{name}/value")]
#[instrument(skip(tag_repo_actor, req))]
pub async fn update_tag_value(
    tag_repo_actor: Data<ActorRef<actor::tag::Message>>,
    name: Path<String>,
    req: Json<UpdateValueRequest>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "request: {} (update_tag_value)", name_ref);

    let tag_value = req.0.into();
    let (command, mut reply) = actor::tag::Message::update_tag_value(name_ref, tag_value);
    tag_repo_actor
        .send_message(command)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = reply.recv().await.ok_or_else(|| {
        tracing::error!("actor response channel closed before reply received");
        actix_web::error::ErrorInternalServerError("Channel closed")
    })?;

    match result {
        Ok(UpdateValueResult::Updated) => Ok(HttpResponse::Ok().json(UpdateValueResponse {
            result: UpdateValueResult::Updated,
        })),
        Ok(UpdateValueResult::Ignored) => Ok(HttpResponse::Ok().json(UpdateValueResponse {
            result: UpdateValueResult::Ignored,
        })),
        Err(UpdateValueError::TagNameNotFound) => {
            tracing::warn!(%request_id, "Tag not found for update: {}", name_ref);
            Ok(HttpResponse::NotFound().body("Tag not found"))
        },
        Err(UpdateValueError::NoneTimestampProvided) => {
            tracing::warn!(%request_id, "Timestamp required for update: {}", name_ref);
            Ok(HttpResponse::BadRequest().body("Timestamp is required after first update"))
        },
        Err(UpdateValueError::TimestamoOutOfOrder {
            previous,
        }) => {
            tracing::warn!(%request_id, "Timestamp out of order for tag: {}", name_ref);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Timestamp out of order",
                "previous_timestamp": previous
            })))
        },
        Err(UpdateValueError::InvalidDataType {
            expected,
            actual,
        }) => {
            tracing::warn!(%request_id, "Invalid data type for tag: {}", name_ref);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid data type",
                "expected": format!("{:?}", expected),
                "actual": format!("{:?}", actual)
            })))
        },
    }
}

#[delete("/tags/{name}")]
#[instrument(skip(tag_repo_actor))]
pub async fn delete_tag(
    tag_repo_actor: Data<ActorRef<actor::tag::Message>>,
    name: Path<String>,
) -> actix_web::Result<HttpResponse> {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "request: {} (delete_tag)", name_ref);

    let (command, mut reply) = actor::tag::Message::delete_tag(name_ref);
    tag_repo_actor
        .send_message(command)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = reply.recv().await.ok_or_else(|| {
        tracing::error!("actor response channel closed before reply received");
        actix_web::error::ErrorInternalServerError("Channel closed")
    })?;

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(DeleteTagError::TagNameNotFound) => {
            tracing::warn!(%request_id, "Tag not found for deletion: {}", name_ref);
            Ok(HttpResponse::NotFound().body("Tag not found"))
        },
    }
}
