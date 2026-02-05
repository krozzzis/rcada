use std::sync::Arc;

use dashmap::DashMap;
use rcada_core::{
    tag::{Tag, TagMeta, TagName, TagValue},
    unit::Unit,
    value::{DataType, Value},
};

use crate::tag_storage::TagRepository;
use crate::tag_storage::command::create_tag::CreateTagResult;
use crate::tag_storage::command::delete_tag::DeleteTagError;
use crate::tag_storage::command::update_value::{UpdateValueError, UpdateValueResult};
use crate::tag_storage::query::read_tag::ReadTagError;

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

impl TagRepository for TagStorage {
    fn tag_exists(&self, name: &TagName) -> bool {
        self.0.values.contains_key(name)
    }

    fn create_tag(&self, name: TagName, unit: Unit, data_type: DataType) -> CreateTagResult {
        if self.0.values.contains_key(&name) {
            return CreateTagResult::AlreadyExists;
        }

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

    fn get_tag(&self, name: &TagName) -> Result<Tag, ReadTagError> {
        let value = self
            .0
            .values
            .get(name)
            .ok_or(ReadTagError::TagNameNotFound)?
            .clone();

        let meta = self
            .0
            .meta
            .get(name)
            .ok_or(ReadTagError::TagNameNotFound)?
            .clone();

        Ok(Tag {
            name: name.clone(),
            value,
            meta,
        })
    }

    fn get_all_tags(&self) -> Vec<Tag> {
        let mut tags = Vec::new();
        for entry in self.0.values.iter() {
            let name = entry.key().clone();
            if let (Some(value), Some(meta)) = (self.0.values.get(&name), self.0.meta.get(&name)) {
                tags.push(Tag {
                    name,
                    value: value.clone(),
                    meta: meta.clone(),
                });
            }
        }
        tags
    }

    fn update_tag_value(
        &self,
        name: TagName,
        value: TagValue,
    ) -> Result<UpdateValueResult, UpdateValueError> {
        let previous_value =
            if let Some(old_value) = self.0.values.insert(name.clone(), value.clone()) {
                old_value
            } else {
                return Err(UpdateValueError::TagNameNotFound);
            };

        if previous_value != value {
            Ok(UpdateValueResult::Updated)
        } else {
            Ok(UpdateValueResult::Ignored)
        }
    }

    fn delete_tag(&self, name: &TagName) -> Result<(), DeleteTagError> {
        let value_removed = self.0.values.remove(name).is_some();
        let _meta_removed = self.0.meta.remove(name).is_some();

        if value_removed {
            Ok(())
        } else {
            Err(DeleteTagError::TagNameNotFound)
        }
    }

    fn get_tag_data_type(&self, name: &TagName) -> Option<DataType> {
        self.0.meta.get(name).map(|meta| meta.data_type)
    }

    fn get_tag_value(&self, name: &TagName) -> Option<TagValue> {
        self.0.values.get(name).map(|value| value.clone())
    }
}
