use std::str::FromStr;

use super::Error;

#[derive(Debug, PartialEq)]
pub enum Type {
    I8,
    U8,
    String,
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "i8" => Self::I8,
            "u8" => Self::U8,
            "string" => Self::String,
            _ => return Err(Error::InvalidType(s.to_owned())),
        })
    }
}
