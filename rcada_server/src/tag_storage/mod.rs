pub mod adapter;
pub mod command;
pub mod query;

use rcada_core::{
    tag::{Tag, TagName, TagValue},
    unit::Unit,
    value::DataType,
};

use crate::tag_storage::command::create_tag::CreateTagResult;
use crate::tag_storage::command::delete_tag::DeleteTagError;
use crate::tag_storage::command::update_value::{UpdateValueError, UpdateValueResult};
use crate::tag_storage::query::read_tag::ReadTagError;

pub trait TagRepository: Clone + Send + Sync + 'static {
    fn tag_exists(&self, name: &TagName) -> bool;

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
}
