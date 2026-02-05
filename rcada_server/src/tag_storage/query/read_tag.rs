use rcada_core::tag::{Tag, TagName};

use crate::tag_storage::TagRepository;

pub struct ReadTagQuery<'a, R: TagRepository> {
    pub tag_name: TagName,
    repo: &'a R,
}

impl<'a, R: TagRepository> ReadTagQuery<'a, R> {
    pub fn new(tag_name: impl Into<TagName>, repo: &'a R) -> Self {
        Self {
            tag_name: tag_name.into(),
            repo,
        }
    }

    pub fn execute(&self) -> Result<Tag, ReadTagError> {
        self.repo.get_tag(&self.tag_name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReadTagError {
    TagNameNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag_storage::adapter::inmemory::TagStorage;
    use crate::tag_storage::command::create_tag::CreateTagCommand;
    use crate::tag_storage::command::update_value::UpdateValueCommand;
    use chrono::Utc;
    use rcada_core::{
        tag::TagValue,
        unit::Unit,
        value::{DataType, Value},
    };

    #[test]
    fn test_read_tag_success() {
        let repo = TagStorage::new();
        let tag_name = "test_tag";

        let create_command =
            CreateTagCommand::new(tag_name, Unit::CelsiusDegree, DataType::Float32, &repo);
        let _ = create_command.execute();

        let read_query = ReadTagQuery::new(tag_name, &repo);
        let result = read_query.execute();

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.name.as_str(), "test_tag");
        assert_eq!(tag.meta.unit, Unit::CelsiusDegree);
        assert_eq!(tag.meta.data_type, DataType::Float32);
        assert_eq!(
            tag.value.value,
            Value::default_with_data_type(DataType::Float32)
        );
        assert_eq!(tag.value.timestamp, None);
    }

    #[test]
    fn test_read_tag_after_update() {
        let repo = TagStorage::new();
        let tag_name = "test_tag";

        let create_command =
            CreateTagCommand::new(tag_name, Unit::CelsiusDegree, DataType::Float32, &repo);
        let _ = create_command.execute();

        let new_value = TagValue {
            value: Value::Float32(25.5),
            timestamp: Some(Utc::now()),
        };

        let update_command = UpdateValueCommand::new(tag_name, new_value, &repo);
        let _ = update_command.execute().unwrap();

        let read_query = ReadTagQuery::new(tag_name, &repo);
        let result = read_query.execute();

        assert!(result.is_ok());
        let read_tag = result.unwrap();
        assert_eq!(read_tag.value.value, Value::Float32(25.5));
        assert!(read_tag.value.timestamp.is_some());
    }

    #[test]
    fn test_read_tag_not_found() {
        let repo = TagStorage::new();
        let tag_name = "nonexistent_tag";

        let read_query = ReadTagQuery::new(tag_name, &repo);
        let result = read_query.execute();

        assert_eq!(result, Err(ReadTagError::TagNameNotFound));
    }

    #[test]
    fn test_read_tag_different_data_types() {
        let repo = TagStorage::new();

        let f32_command =
            CreateTagCommand::new("f32_tag", Unit::CelsiusDegree, DataType::Float32, &repo);
        let _ = f32_command.execute();

        let i32_command = CreateTagCommand::new("i32_tag", Unit::Meter, DataType::Int32, &repo);
        let _ = i32_command.execute();

        let string_command =
            CreateTagCommand::new("string_tag", Unit::None, DataType::String, &repo);
        let _ = string_command.execute();

        let f32_read = ReadTagQuery::new("f32_tag", &repo).execute().unwrap();
        assert_eq!(f32_read.meta.data_type, DataType::Float32);
        assert_eq!(f32_read.meta.unit, Unit::CelsiusDegree);
        assert!(matches!(f32_read.value.value, Value::Float32(_)));

        let i32_read = ReadTagQuery::new("i32_tag", &repo).execute().unwrap();
        assert_eq!(i32_read.meta.data_type, DataType::Int32);
        assert_eq!(i32_read.meta.unit, Unit::Meter);
        assert!(matches!(i32_read.value.value, Value::Int32(_)));

        let string_read = ReadTagQuery::new("string_tag", &repo).execute().unwrap();
        assert_eq!(string_read.meta.data_type, DataType::String);
        assert_eq!(string_read.meta.unit, Unit::None);
        assert!(matches!(string_read.value.value, Value::String(_)));
    }
}
