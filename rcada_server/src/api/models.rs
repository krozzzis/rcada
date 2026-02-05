use chrono::{DateTime, Utc};
use rcada_core::{
    tag::{Tag, TagValue},
    unit::Unit,
    value::{DataType, Value},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub unit: Unit,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTagResponse {
    pub name: String,
    pub result: CreateTagResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateTagResult {
    SuccessfullyCreated,
    AlreadyExists,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateValueRequest {
    pub value: Value,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateValueResponse {
    pub result: UpdateValueResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateValueResult {
    Updated,
    Ignored,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagResponse {
    pub name: String,
    pub value: ValueResponse,
    pub meta: TagMetaResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueResponse {
    pub value: Value,
    pub timestamp: Option<DateTime<Utc>>,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagMetaResponse {
    pub unit: Unit,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTagsResponse {
    pub tags: Vec<TagResponse>,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        let data_type = tag.value.value.get_data_type();
        TagResponse {
            name: tag.name.to_string(),
            value: ValueResponse {
                value: tag.value.value,
                timestamp: tag.value.timestamp,
                data_type,
            },
            meta: TagMetaResponse {
                unit: tag.meta.unit,
                data_type: tag.meta.data_type,
            },
        }
    }
}

impl From<rcada_core::tag::TagValue> for ValueResponse {
    fn from(tag_value: rcada_core::tag::TagValue) -> Self {
        let data_type = tag_value.value.get_data_type();
        ValueResponse {
            value: tag_value.value,
            timestamp: tag_value.timestamp,
            data_type,
        }
    }
}

impl From<UpdateValueRequest> for TagValue {
    fn from(req: UpdateValueRequest) -> Self {
        TagValue {
            value: req.value,
            timestamp: req.timestamp.or(Some(Utc::now())),
        }
    }
}
