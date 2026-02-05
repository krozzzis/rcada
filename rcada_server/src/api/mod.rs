use actix_web::{
    HttpResponse, Responder, delete, get, post, put,
    web::{Data, Json, Path},
};
use tracing::instrument;
use uuid::Uuid;

use crate::container::Container;
use crate::tag_storage::adapter::inmemory::TagStorage;
use crate::tag_storage::command::create_tag::CreateTagResult as CmdCreateTagResult;
use crate::tag_storage::command::delete_tag::DeleteTagError;
use crate::tag_storage::command::update_value::UpdateValueError;
use crate::tag_storage::command::update_value::UpdateValueResult as CmdUpdateValueResult;

use self::models::{
    CreateTagRequest, CreateTagResponse, CreateTagResult, ListTagsResponse, TagResponse,
    UpdateValueRequest, UpdateValueResponse, UpdateValueResult,
};

pub mod models;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/api/v1")
        .service(create_tag)
        .service(list_tags)
        .service(get_tag)
        .service(update_tag_value)
        .service(delete_tag)
}

#[post("/tags")]
#[instrument(skip(container, req))]
pub async fn create_tag(
    container: Data<Container<TagStorage>>,
    req: Json<CreateTagRequest>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    tracing::info!(%request_id, "Creating tag: {}", req.name);

    let command = container.create_create_tag_command(req.name.clone(), req.unit, req.data_type);
    let result = command.execute();

    match result {
        CmdCreateTagResult::SuccessfullyCreated => {
            tracing::info!(%request_id, "Tag created: {}", req.name);
            HttpResponse::Created().json(CreateTagResponse {
                name: req.name.clone(),
                result: CreateTagResult::SuccessfullyCreated,
            })
        }
        CmdCreateTagResult::AlreadyExists => {
            tracing::warn!(%request_id, "Tag already exists: {}", req.name);
            HttpResponse::Conflict().json(CreateTagResponse {
                name: req.name.clone(),
                result: CreateTagResult::AlreadyExists,
            })
        }
    }
}

#[get("/tags")]
#[instrument(skip(container))]
pub async fn list_tags(container: Data<Container<TagStorage>>) -> impl Responder {
    let query = container.create_list_tags_query();
    let result = query.execute();
    let responses: Vec<TagResponse> = result.tags.into_iter().map(|t| t.into()).collect();
    tracing::info!("Listed {} tags", responses.len());
    HttpResponse::Ok().json(ListTagsResponse { tags: responses })
}

#[get("/tags/{name}")]
#[instrument(skip(container))]
pub async fn get_tag(container: Data<Container<TagStorage>>, name: Path<String>) -> impl Responder {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "Getting tag: {}", name_ref);

    let query = container.create_read_tag_query(name_ref);
    let result = query.execute();

    match result {
        Ok(tag) => {
            tracing::info!(%request_id, "Tag found: {}", name_ref);
            HttpResponse::Ok().json(TagResponse::from(tag))
        }
        Err(_) => {
            tracing::warn!(%request_id, "Tag not found: {}", name_ref);
            HttpResponse::NotFound().body("Tag not found")
        }
    }
}

#[put("/tags/{name}/value")]
#[instrument(skip(container, req))]
pub async fn update_tag_value(
    container: Data<Container<TagStorage>>,
    name: Path<String>,
    req: Json<UpdateValueRequest>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "Updating tag: {}", name_ref);

    let tag_value = req.0.into();
    let command = container.create_update_value_command(name_ref, tag_value);
    let result = command.execute();

    match result {
        Ok(CmdUpdateValueResult::Updated) => {
            tracing::info!(%request_id, "Tag updated: {}", name_ref);
            HttpResponse::Ok().json(UpdateValueResponse {
                result: UpdateValueResult::Updated,
            })
        }
        Ok(CmdUpdateValueResult::Ignored) => {
            tracing::debug!(%request_id, "Tag update ignored (same value): {}", name_ref);
            HttpResponse::Ok().json(UpdateValueResponse {
                result: UpdateValueResult::Ignored,
            })
        }
        Err(UpdateValueError::TagNameNotFound) => {
            tracing::warn!(%request_id, "Tag not found for update: {}", name_ref);
            HttpResponse::NotFound().body("Tag not found")
        }
        Err(UpdateValueError::NoneTimestampProvided) => {
            tracing::warn!(%request_id, "Timestamp required for update: {}", name_ref);
            HttpResponse::BadRequest().body("Timestamp is required after first update")
        }
        Err(UpdateValueError::TimestamoOutOfOrder { previous }) => {
            tracing::warn!(%request_id, "Timestamp out of order for tag: {}", name_ref);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Timestamp out of order",
                "previous_timestamp": previous
            }))
        }
        Err(UpdateValueError::InvalidDataType { expected, actual }) => {
            tracing::warn!(%request_id, "Invalid data type for tag: {}", name_ref);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid data type",
                "expected": format!("{:?}", expected),
                "actual": format!("{:?}", actual)
            }))
        }
        Err(UpdateValueError::NoneValueProvided) => {
            tracing::warn!(%request_id, "Value required for update: {}", name_ref);
            HttpResponse::BadRequest().body("Value is required")
        }
    }
}

#[delete("/tags/{name}")]
#[instrument(skip(container))]
pub async fn delete_tag(
    container: Data<Container<TagStorage>>,
    name: Path<String>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let name_ref = name.as_str();
    tracing::info!(%request_id, "Deleting tag: {}", name_ref);

    let command = container.create_delete_tag_command(name_ref);
    let result = command.execute();

    match result {
        Ok(_) => {
            tracing::info!(%request_id, "Tag deleted: {}", name_ref);
            HttpResponse::NoContent().finish()
        }
        Err(DeleteTagError::TagNameNotFound) => {
            tracing::warn!(%request_id, "Tag not found for deletion: {}", name_ref);
            HttpResponse::NotFound().body("Tag not found")
        }
    }
}
