use std::str::FromStr;

use strum::Display;

use super::{AttributeTargetLanguage, AttributeUsedIn, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[strum(serialize_all = "snake_case")]
pub enum AttributeKey {
    As,
    Rename,
    Indent,
}

impl AttributeKey {
    pub const fn allowed_in(&self) -> &[AttributeUsedIn] {
        match self {
            Self::As => &[AttributeUsedIn::Type],
            Self::Rename => &[AttributeUsedIn::TypeField],
            Self::Indent => &[AttributeUsedIn::Root],
        }
    }

    pub const fn allowed_for_target(&self) -> &[AttributeTargetLanguage] {
        match self {
            Self::As => &[AttributeTargetLanguage::Typescript],
            Self::Rename => AttributeTargetLanguage::all(),
            Self::Indent => AttributeTargetLanguage::all(),
        }
    }
}

impl FromStr for AttributeKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "as" => Self::As,
            "rename" => Self::Rename,
            "indent" => Self::Indent,
            _ => return Err(Error::InvalidAttributeKey(s.to_owned())),
        })
    }
}
