use rcada_core::{tag::TagName, unit::Unit, value::DataType};

use crate::tag_storage::TagRepository;
use crate::tag_storage::adapter::inmemory::TagStorage;
use crate::tag_storage::command::create_tag::CreateTagCommand;
use crate::tag_storage::command::create_tag::CreateTagResult;
use crate::tag_storage::command::delete_tag::DeleteTagCommand;
use crate::tag_storage::command::delete_tag::DeleteTagResult;
use crate::tag_storage::command::update_value::UpdateValueCommand;
use crate::tag_storage::query::list_tags::ListTagsQuery;
use crate::tag_storage::query::read_tag::ReadTagQuery;

#[derive(Clone)]
pub struct Container<R: TagRepository> {
    repository: R,
}

impl<R: TagRepository> Container<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_create_tag_command(
        &self,
        name: impl Into<TagName>,
        unit: Unit,
        data_type: DataType,
    ) -> CreateTagCommand<'_, R> {
        CreateTagCommand::new(name, unit, data_type, &self.repository)
    }

    pub fn create_update_value_command(
        &self,
        name: impl Into<TagName>,
        value: rcada_core::tag::TagValue,
    ) -> UpdateValueCommand<'_, R> {
        UpdateValueCommand::new(name, value, &self.repository)
    }

    pub fn create_delete_tag_command(&self, name: impl Into<TagName>) -> DeleteTagCommand<'_, R> {
        DeleteTagCommand::new(name, &self.repository)
    }

    pub fn create_list_tags_query(&self) -> ListTagsQuery<'_, R> {
        ListTagsQuery::new(&self.repository)
    }

    pub fn create_read_tag_query(&self, name: impl Into<TagName>) -> ReadTagQuery<'_, R> {
        ReadTagQuery::new(name, &self.repository)
    }

    pub fn repository(&self) -> &R {
        &self.repository
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let container = Container::new(TagStorage::new());
        let tag_name = TagName::from("test");
        assert!(!container.repository().tag_exists(&tag_name));
    }

    #[test]
    fn test_create_tag_command() {
        let container = Container::new(TagStorage::new());
        let command =
            container.create_create_tag_command("test_tag", Unit::CelsiusDegree, DataType::Float32);
        let result = command.execute();
        assert_eq!(result, CreateTagResult::SuccessfullyCreated);
    }

    #[test]
    fn test_delete_tag_command() {
        let container = Container::new(TagStorage::new());
        let create_cmd =
            container.create_create_tag_command("test_tag", Unit::CelsiusDegree, DataType::Float32);
        let _ = create_cmd.execute();

        let delete_cmd = container.create_delete_tag_command("test_tag");
        let result = delete_cmd.execute();
        assert_eq!(result, Ok(DeleteTagResult::Deleted));
    }
}
