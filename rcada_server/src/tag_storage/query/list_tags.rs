use rcada_core::tag::Tag;

use crate::tag_storage::TagRepository;

pub struct ListTagsQuery<'a, R: TagRepository> {
    repo: &'a R,
}

impl<'a, R: TagRepository> ListTagsQuery<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub fn execute(&self) -> ListTagsResult {
        ListTagsResult {
            tags: self.repo.get_all_tags(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListTagsResult {
    pub tags: Vec<Tag>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag_storage::adapter::inmemory::TagStorage;
    use crate::tag_storage::command::create_tag::CreateTagCommand;
    use rcada_core::unit::Unit;
    use rcada_core::value::DataType;

    #[test]
    fn test_list_tags_empty() {
        let repo = TagStorage::new();
        let query = ListTagsQuery::new(&repo);
        let result = query.execute();
        assert_eq!(result.tags.len(), 0);
    }

    #[test]
    fn test_list_tags_with_tags() {
        let repo = TagStorage::new();

        let create_command1 =
            CreateTagCommand::new("tag1", Unit::CelsiusDegree, DataType::Float32, &repo);
        let _ = create_command1.execute();

        let create_command2 = CreateTagCommand::new("tag2", Unit::Meter, DataType::Int32, &repo);
        let _ = create_command2.execute();

        let query = ListTagsQuery::new(&repo);
        let result = query.execute();
        assert_eq!(result.tags.len(), 2);
    }
}
