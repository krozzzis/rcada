use dashmap::DashMap;
use rcada_core::{
    tag::{Tag, TagMeta, TagName, TagValue},
    unit::Unit,
    value::{DataType, Value},
};

use crate::repository::tag::{
    CreateTagResult, DeleteTagError, ReadTagError, TagRepository, UpdateValueError,
    UpdateValueResult,
};

#[derive(Default, Clone)]
pub struct TagStorage {
    values: DashMap<TagName, TagValue>,
    meta: DashMap<TagName, TagMeta>,
}

impl TagStorage {
    pub fn new() -> Self {
        Self {
            values: DashMap::new(),
            meta: DashMap::new(),
        }
    }
}

impl TagRepository for TagStorage {
    fn is_tag_exists(&self, name: &TagName) -> bool {
        self.values.contains_key(name)
    }

    fn create_tag(&self, name: TagName, unit: Unit, data_type: DataType) -> CreateTagResult {
        if self.values.contains_key(&name) {
            return CreateTagResult::AlreadyExists;
        }

        self.values.insert(
            name.clone(),
            TagValue {
                value: Value::default_with_data_type(data_type),
                timestamp: None,
            },
        );

        self.meta.insert(
            name,
            TagMeta {
                unit,
                data_type,
            },
        );

        CreateTagResult::SuccessfullyCreated
    }

    fn get_tag(&self, name: &TagName) -> Result<Tag, ReadTagError> {
        let value = self
            .values
            .get(name)
            .ok_or(ReadTagError::TagNameNotFound)?
            .clone();

        let meta = self
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
        for entry in self.values.iter() {
            let name = entry.key().clone();
            if let (Some(value), Some(meta)) = (self.values.get(&name), self.meta.get(&name)) {
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
            if let Some(old_value) = self.values.insert(name.clone(), value.clone()) {
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
        let value_removed = self.values.remove(name).is_some();
        let _meta_removed = self.meta.remove(name).is_some();

        if value_removed {
            Ok(())
        } else {
            Err(DeleteTagError::TagNameNotFound)
        }
    }

    fn get_tag_data_type(&self, name: &TagName) -> Option<DataType> {
        self.meta.get(name).map(|meta| meta.data_type)
    }

    fn get_tag_value(&self, name: &TagName) -> Option<TagValue> {
        self.values.get(name).map(|value| value.clone())
    }
}
