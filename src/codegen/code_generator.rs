use std::marker::PhantomData;

use crate::ast::Decl;

use super::codegen_target::CodegenTarget;

pub struct CodeGenerator<T> {
    _marker: PhantomData<T>,
}

impl<T: CodegenTarget> CodeGenerator<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn generate(&self, decls: &[Decl]) -> String {
        let mut out = String::new();

        for decl in decls {
            match decl {
                Decl::Type(type_decl) => T::emit_type_decl(&mut out, type_decl),
                Decl::Enum(enum_decl) => T::emit_enum_decl(&mut out, enum_decl),
            }

            out.push('\n');
        }

        out
    }
}
