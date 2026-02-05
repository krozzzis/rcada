use chrono::{DateTime, Utc};
use rcada_core::{tag::TagName, value::DataType};

pub trait UpdateValueRepository {
    fn get_tag_value(&self, name: &TagName) -> Option<rcada_core::tag::TagValue>;
    fn get_tag_data_type(&self, name: &TagName) -> Option<DataType>;
    fn update_tag_value(
        &self,
        name: TagName,
        value: rcada_core::tag::TagValue,
    ) -> Result<UpdateValueOk, UpdateValueError>;
}

pub struct UpdateValue<R>
where
    R: UpdateValueRepository,
{
    pub name: TagName,
    pub value: rcada_core::tag::TagValue,
    repo: R,
}

impl<R> UpdateValue<R>
where
    R: UpdateValueRepository,
{
    pub fn new(name: impl Into<TagName>, value: rcada_core::tag::TagValue, repo: R) -> Self {
        Self {
            name: name.into(),
            value,
            repo,
        }
    }

    pub fn execute(self) -> Result<UpdateValueOk, UpdateValueError> {
        let data_type = self
            .repo
            .get_tag_data_type(&self.name)
            .ok_or(UpdateValueError::TagNameNotFound)?;

        if self.value.value.get_data_type() != data_type {
            return Err(UpdateValueError::InvalidDataType {
                expected: data_type,
                actual: self.value.value.get_data_type(),
            });
        }

        let storage_value = self
            .repo
            .get_tag_value(&self.name)
            .ok_or(UpdateValueError::TagNameNotFound)?;

        if let Some(timestamp) = storage_value.timestamp {
            if let Some(tag_timestamp) = self.value.timestamp {
                if timestamp >= tag_timestamp {
                    return Err(UpdateValueError::TimestamoOutOfOrder {
                        previous: timestamp,
                    });
                }
            } else {
                return Err(UpdateValueError::NoneTimestampProvided);
            }
        }

        self.repo.update_tag_value(self.name, self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateValueOk {
    Updated,
    Ignored,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateValueError {
    TimestamoOutOfOrder {
        previous: DateTime<Utc>,
    },
    InvalidDataType {
        expected: DataType,
        actual: DataType,
    },
    NoneValueProvided,
    NoneTimestampProvided,
    TagNameNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag_storage::adapter::inmemory::TagStorage;
    use crate::tag_storage::command::create_tag::CreateTag;
    use chrono::Utc;
    use rcada_core::{
        tag::TagName,
        unit::Unit,
        value::{DataType, Value},
    };

    #[test]
    fn test_update_value_success() {
        let repo = TagStorage::new();
        let tag_name = TagName::from("test_tag");

        let create_command = CreateTag::new(
            &*tag_name,
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );
        let _ = create_command.execute();

        let new_value = rcada_core::tag::TagValue {
            value: Value::Float32(25.5),
            timestamp: Some(Utc::now()),
        };

        let update_command = UpdateValue::new(&*tag_name, new_value, repo.clone());
        let result = update_command.execute();
        assert_eq!(result, Ok(UpdateValueOk::Updated));
    }

    #[test]
    fn test_update_value_ignored_when_same() {
        let repo = TagStorage::new();
        let tag_name = TagName::from("test_tag");

        let create_command = CreateTag::new(
            &*tag_name,
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );
        let _ = create_command.execute();

        let timestamp = Utc::now();
        let new_value = rcada_core::tag::TagValue {
            value: Value::Float32(25.5),
            timestamp: Some(timestamp),
        };

        let update_command1 = UpdateValue::new(&*tag_name, new_value.clone(), repo.clone());
        let result1 = update_command1.execute();
        assert_eq!(result1, Ok(UpdateValueOk::Updated));

        let later_timestamp = timestamp + chrono::Duration::milliseconds(1);
        let same_value = rcada_core::tag::TagValue {
            value: Value::Float32(25.5),
            timestamp: Some(later_timestamp),
        };

        let update_command2 = UpdateValue::new(&*tag_name, same_value, repo.clone());
        let result2 = update_command2.execute();

        assert!(result2.is_ok());
        assert!(matches!(
            result2,
            Ok(UpdateValueOk::Updated) | Ok(UpdateValueOk::Ignored)
        ));
    }

    #[test]
    fn test_update_value_tag_not_found() {
        let repo = TagStorage::new();
        let tag_name = TagName::from("nonexistent_tag");

        let new_value = rcada_core::tag::TagValue {
            value: Value::Float32(25.5),
            timestamp: Some(Utc::now()),
        };

        let update_command = UpdateValue::new(&*tag_name, new_value, repo.clone());
        let result = update_command.execute();
        assert_eq!(result, Err(UpdateValueError::TagNameNotFound));
    }

    #[test]
    fn test_update_value_timestamp_out_of_order() {
        let repo = TagStorage::new();
        let tag_name = TagName::from("test_tag");

        let create_command = CreateTag::new(
            &*tag_name,
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );
        let _ = create_command.execute();

        let later_timestamp = Utc::now();
        let first_value = rcada_core::tag::TagValue {
            value: Value::Float32(25.5),
            timestamp: Some(later_timestamp),
        };

        let update_command1 = UpdateValue::new(&*tag_name, first_value, repo.clone());
        let result1 = update_command1.execute();
        assert_eq!(result1, Ok(UpdateValueOk::Updated));

        let earlier_timestamp = later_timestamp - chrono::Duration::seconds(1);
        let second_value = rcada_core::tag::TagValue {
            value: Value::Float32(30.0),
            timestamp: Some(earlier_timestamp),
        };

        let update_command2 = UpdateValue::new(&*tag_name, second_value, repo.clone());
        let result2 = update_command2.execute();
        assert!(matches!(
            result2,
            Err(UpdateValueError::TimestamoOutOfOrder { .. })
        ));
    }

    #[test]
    fn test_update_value_none_timestamp_provided() {
        let repo = TagStorage::new();
        let tag_name = TagName::from("test_tag");

        let create_command = CreateTag::new(
            &*tag_name,
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );
        let _ = create_command.execute();

        let first_value = rcada_core::tag::TagValue {
            value: Value::Float32(25.5),
            timestamp: Some(Utc::now()),
        };

        let update_command1 = UpdateValue::new(&*tag_name, first_value, repo.clone());
        let result1 = update_command1.execute();
        assert_eq!(result1, Ok(UpdateValueOk::Updated));

        let second_value = rcada_core::tag::TagValue {
            value: Value::Float32(30.0),
            timestamp: None,
        };

        let update_command2 = UpdateValue::new(&*tag_name, second_value, repo.clone());
        let result2 = update_command2.execute();
        assert_eq!(result2, Err(UpdateValueError::NoneTimestampProvided));
    }

    #[test]
    fn test_update_value_invalid_data_type() {
        let repo = TagStorage::new();
        let tag_name = TagName::from("test_tag");

        let create_command = CreateTag::new(
            &*tag_name,
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );
        let _ = create_command.execute();

        let invalid_value = rcada_core::tag::TagValue {
            value: Value::Int32(25),
            timestamp: Some(Utc::now()),
        };

        let update_command = UpdateValue::new(&*tag_name, invalid_value, repo.clone());
        let result = update_command.execute();
        assert!(matches!(
            result,
            Err(UpdateValueError::InvalidDataType {
                expected: DataType::Float32,
                actual: DataType::Int32
            })
        ));
    }
}
