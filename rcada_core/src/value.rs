use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Float(f32),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
}

impl Value {
    pub fn get_data_type(&self) -> DataType {
        match self {
            Value::Integer(_) => DataType::Integer,
            Value::Float(_) => DataType::Float,
            Value::Boolean(_) => DataType::Boolean,
            Value::String(_) => DataType::String,
        }
    }

    pub fn default_with_data_type(data_type: DataType) -> Self {
        match data_type {
            DataType::Integer => Self::Integer(Default::default()),
            DataType::Float => Self::Float(Default::default()),
            DataType::Boolean => Self::Boolean(Default::default()),
            DataType::String => Self::String(Default::default()),
        }
    }
}
