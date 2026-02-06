pub mod inmemory;
pub mod message;

pub use message::Message;

use rcada_core::{
    tag::{Tag, TagName, TagValue},
    unit::Unit,
    value::DataType,
};

use crate::tag_storage::message::{
    CreateTagResult, DeleteTagError, ReadTagError, UpdateValueError, UpdateValueResult,
};

pub trait TagRepository: Send + Sync {
    fn is_tag_exists(&self, name: &TagName) -> bool;

    fn create_tag(&self, name: TagName, unit: Unit, data_type: DataType) -> CreateTagResult;

    fn get_tag(&self, name: &TagName) -> Result<Tag, ReadTagError>;

    fn get_all_tags(&self) -> Vec<Tag>;

    fn update_tag_value(
        &self,
        name: TagName,
        value: TagValue,
    ) -> Result<UpdateValueResult, UpdateValueError>;

    fn delete_tag(&self, name: &TagName) -> Result<(), DeleteTagError>;

    fn get_tag_data_type(&self, name: &TagName) -> Option<DataType>;

    fn get_tag_value(&self, name: &TagName) -> Option<TagValue>;

    fn execute(&self, message: Message) -> bool {
        match message {
            Message::CreateTag {
                name,
                unit,
                data_type,
                result,
            } => result.send(self.create_tag(name, unit, data_type)).is_ok(),
            Message::UpdateTagValue {
                name,
                value,
                result,
            } => result.send(self.update_tag_value(name, value)).is_ok(),
            Message::DeleteTag { name, result } => result.send(self.delete_tag(&name)).is_ok(),
            Message::TagExists { name, result } => result.send(self.is_tag_exists(&name)).is_ok(),
            Message::GetTag { name, result } => result.send(self.get_tag(&name)).is_ok(),
            Message::GetAllTags { result } => result.send(self.get_all_tags()).is_ok(),
            Message::GetTagDataType { name, result } => {
                result.send(self.get_tag_data_type(&name)).is_ok()
            }
            Message::GetTagValue { name, result } => result.send(self.get_tag_value(&name)).is_ok(),
        }
    }
}
