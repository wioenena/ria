use std::str::FromStr;

use strum::Display;

use super::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[strum(serialize_all = "snake_case")]
pub enum AttributeTargetLanguage {
    Typescript,
    Rust,
    Go,
}

impl AttributeTargetLanguage {
    pub const fn all() -> &'static [AttributeTargetLanguage] {
        &[Self::Typescript, Self::Rust, Self::Go]
    }
}

impl FromStr for AttributeTargetLanguage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "typescript" => Self::Typescript,
            "rust" => Self::Rust,
            "go" => Self::Go,
            _ => return Err(Error::InvalidTarget(s.to_owned())),
        })
    }
}
