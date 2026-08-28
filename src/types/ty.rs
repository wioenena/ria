#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    F32,
    F64,
    Bool,
    String,
    Custom(String),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "i8" => Self::I8,
            "u8" => Self::U8,
            "i16" => Self::I16,
            "u16" => Self::U16,
            "i32" => Self::I32,
            "u32" => Self::U32,
            "i64" => Self::I64,
            "u64" => Self::U64,
            "i128" => Self::I128,
            "u128" => Self::U128,
            "f32" => Self::F32,
            "f64" => Self::F64,
            "bool" => Self::Bool,
            "string" => Self::String,
            other => Self::Custom(other.to_owned()),
        }
    }
}
