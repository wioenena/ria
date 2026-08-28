#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown type: `{name}` referenced in `{used_in}`")]
    UnknownType { name: String, used_in: String },
    #[error("`{name}` is declared more than once")]
    DuplicateDeclaration { name: String },
}
