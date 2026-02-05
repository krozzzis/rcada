#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Float32(f32),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    UInt8,
    UInt16,
    UInt32,
    Int8,
    Int16,
    Int32,
    Float32,
    Boolean,
    String,
}

impl Value {
    pub fn get_data_type(&self) -> DataType {
        match self {
            Value::UInt8(_) => DataType::UInt8,
            Value::UInt16(_) => DataType::UInt16,
            Value::UInt32(_) => DataType::UInt32,
            Value::Int8(_) => DataType::Int8,
            Value::Int16(_) => DataType::Int16,
            Value::Int32(_) => DataType::Int32,
            Value::Float32(_) => DataType::Float32,
            Value::Boolean(_) => DataType::Boolean,
            Value::String(_) => DataType::String,
        }
    }

    pub fn default_with_data_type(data_type: DataType) -> Self {
        match data_type {
            DataType::UInt8 => Self::UInt8(Default::default()),
            DataType::UInt16 => Self::UInt16(Default::default()),
            DataType::UInt32 => Self::UInt32(Default::default()),
            DataType::Int8 => Self::Int8(Default::default()),
            DataType::Int16 => Self::Int16(Default::default()),
            DataType::Int32 => Self::Int32(Default::default()),
            DataType::Float32 => Self::Float32(Default::default()),
            DataType::Boolean => Self::Boolean(Default::default()),
            DataType::String => Self::String(Default::default()),
        }
    }
}
