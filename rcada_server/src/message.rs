use crate::tag_storage;

pub enum Message {
    Tag(tag_storage::Message),
}

impl From<tag_storage::Message> for Message {
    fn from(value: tag_storage::Message) -> Self {
        Self::Tag(value)
    }
}
