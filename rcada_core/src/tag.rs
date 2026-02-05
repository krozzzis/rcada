use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{
    unit::Unit,
    value::{DataType, Value},
};

pub type TagName = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TagValue {
    pub value: Value,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TagMeta {
    pub unit: Unit,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    pub name: TagName,
    pub value: TagValue,
    pub meta: TagMeta,
}
