use crate::ast::attribute::{self, AttributeKey, AttributeTargetLanguage, AttributeUsedIn};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown type: `{name}` at line {line}, column: {column}")]
    UnknownType {
        name: String,
        used_in: String,
        line: usize,
        column: usize,
    },
    #[error("`{name}` is declared more than once")]
    DuplicateDeclaration { name: String },

    #[error(
        "attribute target `{target}` is not allowed for `{key}` here at {line}:{column}, allowed targets: {allowed:?}"
    )]
    UnsupportedAttributeTarget {
        target: AttributeTargetLanguage,
        key: AttributeKey,
        allowed: Vec<AttributeTargetLanguage>,
        line: usize,
        column: usize,
    },
    #[error(
        "attribute key `{key}` is not allowed in `{used_in}` at {line}:{column}, allowed contexts: {allowed:?}"
    )]
    DisallowedAttributeUsage {
        key: AttributeKey,
        used_in: attribute::AttributeUsedIn,
        allowed: Vec<AttributeUsedIn>,
        line: usize,
        column: usize,
    },
}
