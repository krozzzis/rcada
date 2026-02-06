use std::sync::Arc;

use crate::{message::Message, tag_storage::TagRepository};

const BUFFER_SIZE: usize = 128;

#[derive(Clone)]
pub struct SystemBroker<TR: TagRepository> {
    tag_repository: Arc<TR>,
}

impl<TR: TagRepository> SystemBroker<TR> {
    pub fn new(tag_repository: TR) -> Self {
        Self {
            tag_repository: Arc::new(tag_repository),
        }
    }

    pub fn send(&self, message: impl Into<Message>) -> bool {
        match message.into() {
            Message::Tag(message) => self.tag_repository.execute(message),
        }
    }
}
