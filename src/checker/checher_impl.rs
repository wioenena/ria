use std::collections::HashMap;

use crate::ast::{Decl, EnumDecl, Program, TypeDecl};
use crate::types::Type;

use super::Error;
use super::symbol_kind::SymbolKind;

#[derive(Debug, Default)]
pub struct Checker {
    symbols: HashMap<String, SymbolKind>,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn check(mut self, program: &Program) -> Result<(), Error> {
        self.collect_symbols(program.decls())?;
        self.check_types(program.decls())?;
        self.check_attributes(program)?;
        Ok(())
    }

    fn collect_symbols(&mut self, decls: &[Decl]) -> Result<(), Error> {
        for decl in decls {
            let (name, kind) = match decl {
                Decl::Type(t) => (t.name(), SymbolKind::Struct),
                Decl::Enum(e) => (e.name(), SymbolKind::Enum),
            };

            if self.symbols.insert(name.to_owned(), kind).is_some() {
                return Err(Error::DuplicateDeclaration {
                    name: name.to_owned(),
                });
            }
        }

        Ok(())
    }

    fn check_types(&self, decls: &[Decl]) -> Result<(), Error> {
        for decl in decls {
            let Decl::Type(type_decl) = decl else {
                continue;
            };

            for field in type_decl.fields() {
                if let Type::Custom(name) = field.ty()
                    && !self.symbols.contains_key(name)
                {
                    return Err(Error::UnknownType {
                        name: name.clone(),
                        used_in: format!("{}.{}", type_decl.name(), field.name()),
                        line: field.line(),
                        column: field.column(),
                    });
                }
            }
        }

        Ok(())
    }

    fn check_attributes(&self, program: &Program) -> Result<(), Error> {
        self.check_program_attributes(program)?;
        self.check_decl_attributes(program.decls())?;

        Ok(())
    }

    fn check_program_attributes(&self, program: &Program) -> Result<(), Error> {
        for (target_lang, attrs) in program.attribute_container().get_attributes() {
            for (key, attr) in attrs {
                if !key.allowed_for_target().contains(target_lang) {
                    return Err(Error::UnsupportedAttributeTarget {
                        target: *target_lang,
                        key: *key,
                        allowed: key.allowed_for_target().to_vec(),
                        line: attr.line(),
                        column: attr.column(),
                    });
                }

                if !key.allowed_in().contains(&attr.used_in()) {
                    return Err(Error::DisallowedAttributeUsage {
                        key: *key,
                        used_in: attr.used_in(),
                        allowed: key.allowed_in().to_vec(),
                        line: attr.line(),
                        column: attr.column(),
                    });
                }
            }
        }

        Ok(())
    }

    fn check_decl_attributes(&self, decl: &[Decl]) -> Result<(), Error> {
        for decl in decl {
            match decl {
                Decl::Type(t) => self.check_type_decl_attributes(t)?,
                Decl::Enum(e) => self.check_enum_decl_attributes(e)?,
            }
        }

        Ok(())
    }

    fn check_type_decl_attributes(&self, decl: &TypeDecl) -> Result<(), Error> {
        for (target_lang, attrs) in decl.attribute_container().get_attributes() {
            for (key, attr) in attrs {
                if !key.allowed_for_target().contains(target_lang) {
                    return Err(Error::UnsupportedAttributeTarget {
                        target: *target_lang,
                        key: *key,
                        allowed: key.allowed_for_target().to_vec(),
                        line: attr.line(),
                        column: attr.column(),
                    });
                }

                if !key.allowed_in().contains(&attr.used_in()) {
                    return Err(Error::DisallowedAttributeUsage {
                        key: *key,
                        used_in: attr.used_in(),
                        allowed: key.allowed_in().to_vec(),
                        line: attr.line(),
                        column: attr.column(),
                    });
                }
            }
        }
        Ok(())
    }

    fn check_enum_decl_attributes(&self, _decl: &EnumDecl) -> Result<(), Error> {
        println!("TODO: add enum attribute support");
        Ok(())
    }
}
