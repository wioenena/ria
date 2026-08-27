use super::enum_decl::EnumDecl;
use super::type_decl::TypeDecl;

#[derive(Debug, PartialEq)]
pub enum Decl {
    Enum(EnumDecl),
    Type(TypeDecl),
}
