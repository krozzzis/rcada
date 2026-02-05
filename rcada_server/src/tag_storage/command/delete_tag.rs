use rcada_core::tag::TagName;

use crate::tag_storage::TagRepository;

pub struct DeleteTagCommand<'a, R: TagRepository> {
    pub name: TagName,
    repo: &'a R,
}

impl<'a, R: TagRepository> DeleteTagCommand<'a, R> {
    pub fn new(name: impl Into<TagName>, repo: &'a R) -> Self {
        Self {
            name: name.into(),
            repo,
        }
    }

    pub fn execute(self) -> Result<DeleteTagResult, DeleteTagError> {
        self.repo.delete_tag(&self.name)?;
        Ok(DeleteTagResult::Deleted)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteTagResult {
    Deleted,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteTagError {
    TagNameNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag_storage::adapter::inmemory::TagStorage;
    use crate::tag_storage::command::create_tag::CreateTagCommand;
    use rcada_core::{unit::Unit, value::DataType};

    #[test]
    fn test_delete_tag_success() {
        let repo = TagStorage::new();

        let create_command =
            CreateTagCommand::new("test_tag", Unit::CelsiusDegree, DataType::Float32, &repo);
        let _ = create_command.execute();

        let delete_command = DeleteTagCommand::new("test_tag", &repo);
        let result = delete_command.execute();
        assert_eq!(result, Ok(DeleteTagResult::Deleted));
    }

    #[test]
    fn test_delete_tag_not_found() {
        let repo = TagStorage::new();

        let delete_command = DeleteTagCommand::new("nonexistent_tag", &repo);
        let result = delete_command.execute();
        assert_eq!(result, Err(DeleteTagError::TagNameNotFound));
    }
}
