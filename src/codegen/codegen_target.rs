use crate::ast::{EnumDecl, TypeDecl};
use crate::types::Type;

pub trait CodegenTarget {
    fn type_name(ty: &Type) -> &str;
    fn emit_type_decl(out: &mut String, type_decl: &TypeDecl);
    fn emit_enum_decl(out: &mut String, enum_decl: &EnumDecl);
}
