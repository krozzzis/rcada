use rcada_core::{tag::TagName, unit::Unit, value::DataType};

pub trait CreateTagRepository {
    fn tag_exists(&self, name: &TagName) -> bool;
    fn insert_tag(&self, name: TagName, unit: Unit, data_type: DataType) -> CreateTagResult;
}

pub struct CreateTag<R>
where
    R: CreateTagRepository,
{
    pub name: TagName,
    pub unit: Unit,
    pub data_type: DataType,
    repo: R,
}

impl<R> CreateTag<R>
where
    R: CreateTagRepository,
{
    pub fn new(name: impl Into<TagName>, unit: Unit, data_type: DataType, repo: R) -> Self {
        Self {
            name: name.into(),
            unit,
            data_type,
            repo,
        }
    }

    pub fn execute(self) -> CreateTagResult {
        // Check if tag already exists
        if self.repo.tag_exists(&self.name) {
            return CreateTagResult::AlreadyExists;
        }

        // Create the tag
        self.repo.insert_tag(self.name, self.unit, self.data_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CreateTagResult {
    SuccessfullyCreated,
    AlreadyExists,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag_storage::adapter::inmemory::TagStorage;
    use rcada_core::{unit::Unit, value::DataType};

    #[test]
    fn test_create_tag_success() {
        let repo = TagStorage::new();
        let command = CreateTag::new(
            "test_tag",
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );

        let result = command.execute();
        assert_eq!(result, CreateTagResult::SuccessfullyCreated);
    }

    #[test]
    fn test_create_tag_already_exists() {
        let repo = TagStorage::new();
        let tag_name = "duplicate_tag";

        let command1 = CreateTag::new(
            tag_name,
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );
        let result1 = command1.execute();
        assert_eq!(result1, CreateTagResult::SuccessfullyCreated);

        let command2 = CreateTag::new(tag_name, Unit::Meter, DataType::Int32, repo.clone());
        let result2 = command2.execute();
        assert_eq!(result2, CreateTagResult::AlreadyExists);
    }

    #[test]
    fn test_create_tag_different_types() {
        let repo = TagStorage::new();

        let f32_command = CreateTag::new(
            "f32_tag",
            Unit::CelsiusDegree,
            DataType::Float32,
            repo.clone(),
        );
        let f32_result = f32_command.execute();
        assert_eq!(f32_result, CreateTagResult::SuccessfullyCreated);

        let i32_command = CreateTag::new("i32_tag", Unit::Meter, DataType::Int32, repo.clone());
        let i32_result = i32_command.execute();
        assert_eq!(i32_result, CreateTagResult::SuccessfullyCreated);

        let string_command =
            CreateTag::new("string_tag", Unit::None, DataType::String, repo.clone());
        let string_result = string_command.execute();
        assert_eq!(string_result, CreateTagResult::SuccessfullyCreated);
    }
}
