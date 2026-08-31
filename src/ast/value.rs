#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(u32),
}

pub trait FromValue: Sized {
    fn from_value(value: &Value) -> Option<Self>;
}

impl FromValue for String {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl FromValue for u32 {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
}
