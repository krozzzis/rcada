use chrono::{DateTime, Utc};
use smol_str::SmolStr;

use crate::{
    unit::Unit,
    value::{DataType, Value},
};

pub type TagName = SmolStr;

#[derive(Debug, Clone, PartialEq)]
pub struct TagValue {
    pub value: Value,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TagMeta {
    pub unit: Unit,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub name: TagName,
    pub value: TagValue,
    pub meta: TagMeta,
}
