use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

use rcada_core::{
    tag::{Tag, TagName, TagValue},
    unit::Unit,
    value::DataType,
};

#[derive(Debug)]
pub enum Message {
    CreateTag {
        name: TagName,
        unit: Unit,
        data_type: DataType,
        result: oneshot::Sender<CreateTagResult>,
    },
    UpdateTagValue {
        name: TagName,
        value: TagValue,
        result: oneshot::Sender<Result<UpdateValueResult, UpdateValueError>>,
    },
    DeleteTag {
        name: TagName,
        result: oneshot::Sender<Result<(), DeleteTagError>>,
    },
    TagExists {
        name: TagName,
        result: oneshot::Sender<bool>,
    },
    GetTag {
        name: TagName,
        result: oneshot::Sender<Result<Tag, ReadTagError>>,
    },
    GetAllTags {
        result: oneshot::Sender<Vec<Tag>>,
    },
    GetTagDataType {
        name: TagName,
        result: oneshot::Sender<Option<DataType>>,
    },
    GetTagValue {
        name: TagName,
        result: oneshot::Sender<Option<TagValue>>,
    },
}

impl Message {
    pub fn create_tag(
        name: impl Into<TagName>,
        unit: Unit,
        data_type: DataType,
    ) -> (Self, oneshot::Receiver<CreateTagResult>) {
        let (sender, receiver) = oneshot::channel();
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
        oneshot::Receiver<Result<UpdateValueResult, UpdateValueError>>,
    ) {
        let (sender, receiver) = oneshot::channel();
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
    ) -> (Self, oneshot::Receiver<Result<(), DeleteTagError>>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::DeleteTag {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_tag(
        name: impl Into<TagName>,
    ) -> (Self, oneshot::Receiver<Result<Tag, ReadTagError>>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::GetTag {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_all_tags() -> (Self, oneshot::Receiver<Vec<Tag>>) {
        let (sender, receiver) = oneshot::channel();
        (Self::GetAllTags { result: sender }, receiver)
    }

    pub fn tag_exists(name: impl Into<TagName>) -> (Self, oneshot::Receiver<bool>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::TagExists {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_tag_value(name: impl Into<TagName>) -> (Self, oneshot::Receiver<Option<TagValue>>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::GetTagValue {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }

    pub fn get_tag_data_type(
        name: impl Into<TagName>,
    ) -> (Self, oneshot::Receiver<Option<DataType>>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::GetTagDataType {
                name: name.into(),
                result: sender,
            },
            receiver,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateTagResult {
    SuccessfullyCreated,
    AlreadyExists,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateValueResult {
    Updated,
    Ignored,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateValueError {
    TimestamoOutOfOrder {
        previous: DateTime<Utc>,
    },
    InvalidDataType {
        expected: DataType,
        actual: DataType,
    },
    NoneTimestampProvided,
    TagNameNotFound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteTagResult {
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteTagError {
    TagNameNotFound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReadTagError {
    TagNameNotFound,
}
