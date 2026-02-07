use std::marker::PhantomData;
use std::sync::Arc;

use ractor::ActorProcessingErr;
use ractor::{Actor, ActorRef};
use tokio::sync::mpsc;

use rcada_core::{
    tag::{Tag, TagName, TagValue},
    unit::Unit,
    value::DataType,
};

const REPLY_CHANNEL_SIZE: usize = 1;

use crate::repository::tag::{
    CreateTagResult, DeleteTagError, ReadTagError, TagRepository, UpdateValueError,
    UpdateValueResult,
};

#[derive(Default)]
pub struct TagRepositoryActor<R: TagRepository + Default> {
    _repo: PhantomData<R>,
}

#[cfg_attr(feature = "async-trait", ractor::async_trait)]
impl<R> Actor for TagRepositoryActor<R>
where
    R: TagRepository + Default + 'static,
{
    type Msg = Message;
    type State = Arc<R>;
    type Arguments = R;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        repo: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        tracing::info!("actor: TagRepository started");
        Ok(Arc::new(repo))
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        tracing::info!("handling message {message:?}");
        let ok = match message {
            Message::CreateTag {
                name,
                unit,
                data_type,
                result,
            } => result
                .send(state.create_tag(name, unit, data_type))
                .await
                .is_ok(),
            Message::UpdateTagValue {
                name,
                value,
                result,
            } => result
                .send(state.update_tag_value(name, value))
                .await
                .is_ok(),
            Message::DeleteTag {
                name,
                result,
            } => result.send(state.delete_tag(&name)).await.is_ok(),
            Message::TagExists {
                name,
                result,
            } => result.send(state.is_tag_exists(&name)).await.is_ok(),
            Message::GetTag {
                name,
                result,
            } => result.send(state.get_tag(&name)).await.is_ok(),
            Message::GetAllTags {
                result,
            } => result.send(state.get_all_tags()).await.is_ok(),
            Message::GetTagDataType {
                name,
                result,
            } => result.send(state.get_tag_data_type(&name)).await.is_ok(),
            Message::GetTagValue {
                name,
                result,
            } => result.send(state.get_tag_value(&name)).await.is_ok(),
        };
        if ok {
            Ok(())
        } else {
            tracing::error!("failed to send result to channel (receiver dropped)");
            Err("Cannot send result to channel".into())
        }
    }
}

#[derive(Debug)]
pub enum Message {
    CreateTag {
        name: TagName,
        unit: Unit,
        data_type: DataType,
        result: mpsc::Sender<CreateTagResult>,
    },
    UpdateTagValue {
        name: TagName,
        value: TagValue,
        result: mpsc::Sender<Result<UpdateValueResult, UpdateValueError>>,
    },
    DeleteTag {
        name: TagName,
        result: mpsc::Sender<Result<(), DeleteTagError>>,
    },
    TagExists {
        name: TagName,
        result: mpsc::Sender<bool>,
    },
    GetTag {
        name: TagName,
        result: mpsc::Sender<Result<Tag, ReadTagError>>,
    },
    GetAllTags {
        result: mpsc::Sender<Vec<Tag>>,
    },
    GetTagDataType {
        name: TagName,
        result: mpsc::Sender<Option<DataType>>,
    },
    GetTagValue {
        name: TagName,
        result: mpsc::Sender<Option<TagValue>>,
    },
}

#[cfg(feature = "cluster")]
impl ractor::Message for Message {}

impl Message {
    pub fn create_tag(
        name: impl Into<TagName>,
        unit: Unit,
        data_type: DataType,
    ) -> (Self, mpsc::Receiver<CreateTagResult>) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::CreateTag {
                name: name.into(),
                unit,
                data_type,
                result: sender,
            },
            receiver,
        )
    }

    pub fn update_tag_value(
        name: impl Into<TagName>,
        value: TagValue,
    ) -> (
        Self,
        mpsc::Receiver<Result<UpdateValueResult, UpdateValueError>>,
    ) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::UpdateTagValue {
                name: name.into(),
                value,
                result: sender,
            },
            receiver,
        )
    }

    pub fn delete_tag(
        name: impl Into<TagName>,
    ) -> (Self, mpsc::Receiver<Result<(), DeleteTagError>>) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::DeleteTag {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_tag(name: impl Into<TagName>) -> (Self, mpsc::Receiver<Result<Tag, ReadTagError>>) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::GetTag {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_all_tags() -> (Self, mpsc::Receiver<Vec<Tag>>) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::GetAllTags {
                result: sender,
            },
            receiver,
        )
    }

    pub fn tag_exists(name: impl Into<TagName>) -> (Self, mpsc::Receiver<bool>) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::TagExists {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_tag_value(name: impl Into<TagName>) -> (Self, mpsc::Receiver<Option<TagValue>>) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::GetTagValue {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_tag_data_type(name: impl Into<TagName>) -> (Self, mpsc::Receiver<Option<DataType>>) {
        let (sender, receiver) = mpsc::channel(REPLY_CHANNEL_SIZE);
        (
            Self::GetTagDataType {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }
}
