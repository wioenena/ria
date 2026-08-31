use strum::Display;

#[derive(Debug, Clone, Copy, PartialEq, Display)]
#[strum(serialize_all = "snake_case")]
pub enum AttributeUsedIn {
    Root,
    Type,
    TypeField,
    Enum,
}
