pub mod inmemory;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use rcada_core::{
    tag::{Tag, TagName, TagValue},
    unit::Unit,
    value::DataType,
};

pub trait TagRepository: Send + Sync + Sized {
    fn is_tag_exists(&self, name: &TagName) -> bool;

    fn create_tag(&self, name: TagName, unit: Unit, data_type: DataType) -> CreateTagResult;

    fn get_tag(&self, name: &TagName) -> Result<Tag, ReadTagError>;

    fn get_all_tags(&self) -> Vec<Tag>;

    fn update_tag_value(
        &self,
        name: TagName,
        value: TagValue,
    ) -> Result<UpdateValueResult, UpdateValueError>;

    fn delete_tag(&self, name: &TagName) -> Result<(), DeleteTagError>;

    fn get_tag_data_type(&self, name: &TagName) -> Option<DataType>;

    fn get_tag_value(&self, name: &TagName) -> Option<TagValue>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateTagResult {
    SuccessfullyCreated,
    AlreadyExists,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateValueResult {
    Updated,
    Ignored,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateValueError {
    TimestamoOutOfOrder {
        previous: DateTime<Utc>,
    },
    InvalidDataType {
        expected: DataType,
        actual: DataType,
    },
    NoneTimestampProvided,
    TagNameNotFound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteTagResult {
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteTagError {
    TagNameNotFound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReadTagError {
    TagNameNotFound,
}
