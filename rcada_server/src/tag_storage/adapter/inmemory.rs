use std::sync::Arc;

use dashmap::DashMap;
use rcada_core::{
    tag::{Tag, TagMeta, TagName, TagValue},
    unit::Unit,
    value::{DataType, Value},
};

use crate::tag_storage::{
    command::{
        create_tag::{CreateTagRepository, CreateTagResult},
        update_value::{UpdateValueError, UpdateValueOk, UpdateValueRepository},
    },
    query::read_tag::{ReadTagError, ReadTagRepository},
};

#[derive(Clone)]
pub struct TagStorage(Arc<TagStorageInner>);

struct TagStorageInner {
    values: DashMap<TagName, TagValue>,
    meta: DashMap<TagName, TagMeta>,
}

impl TagStorage {
    pub fn new() -> Self {
        Self(Arc::new(TagStorageInner {
            values: DashMap::new(),
            meta: DashMap::new(),
        }))
    }
}

impl CreateTagRepository for TagStorage {
    fn tag_exists(&self, name: &TagName) -> bool {
        self.0.values.contains_key(name)
    }

    fn insert_tag(&self, name: TagName, unit: Unit, data_type: DataType) -> CreateTagResult {
        self.0.values.insert(
            name.clone(),
            TagValue {
                value: Value::default_with_data_type(data_type),
                timestamp: None,
            },
        );

        self.0.meta.insert(name, TagMeta { unit, data_type });

        CreateTagResult::SuccessfullyCreated
    }
}

impl UpdateValueRepository for TagStorage {
    fn get_tag_value(&self, name: &TagName) -> Option<TagValue> {
        self.0.values.get(name).map(|value| value.clone())
    }

    fn get_tag_data_type(&self, name: &TagName) -> Option<DataType> {
        self.0.meta.get(name).map(|meta| meta.data_type)
    }

    fn update_tag_value(
        &self,
        name: TagName,
        value: TagValue,
    ) -> Result<UpdateValueOk, UpdateValueError> {
        let previous_value =
            if let Some(old_value) = self.0.values.insert(name.clone(), value.clone()) {
                old_value
            } else {
                return Err(UpdateValueError::TagNameNotFound);
            };

        if previous_value != value {
            Ok(UpdateValueOk::Updated)
        } else {
            Ok(UpdateValueOk::Ignored)
        }
    }
}

impl ReadTagRepository for TagStorage {
    fn get_value(&self, tag_name: &TagName) -> Result<Tag, ReadTagError> {
        let value = self
            .0
            .values
            .get(tag_name)
            .ok_or(ReadTagError::TagNameNotFound)?
            .clone();

        let meta = self
            .0
            .meta
            .get(tag_name)
            .ok_or(ReadTagError::TagNameNotFound)?
            .clone();

        Ok(Tag {
            name: tag_name.clone(),
            value,
            meta,
        })
    }
}
