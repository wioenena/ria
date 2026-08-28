use crate::types::Type;

use super::codegen_target::CodegenTarget;

pub struct Typescript;

impl CodegenTarget for Typescript {
    fn type_name(ty: &crate::types::Type) -> &str {
        match ty {
            Type::Bool => "boolean",
            Type::String => "string",
            Type::I128 | Type::U128 | Type::I64 | Type::U64 => "bigint",
            Type::Custom(name) => name.as_str(),
            _ => "number",
        }
    }

    fn emit_type_decl(out: &mut String, type_decl: &crate::ast::TypeDecl) {
        out.push_str(&format!("export interface {} {{\n", type_decl.name()));

        for field in type_decl.fields() {
            out.push_str(&format!(
                "\t{}: {};\n",
                field.name(),
                Self::type_name(field.ty())
            ));
        }

        out.push_str("}\n");
    }

    fn emit_enum_decl(out: &mut String, enum_decl: &crate::ast::EnumDecl) {
        out.push_str(&format!("export enum {} {{\n", enum_decl.name()));

        for v in enum_decl.values() {
            out.push_str(&format!("\t{v},\n"));
        }

        out.push_str("}\n");
    }
}
