use std::collections::HashMap;

use crate::ast::Decl;
use crate::types::Type;

use super::Error;
use super::symbol_kind::SymbolKind;

#[derive(Debug)]
pub struct Resolver<'a> {
    decls: &'a [Decl],
    symbols: HashMap<String, SymbolKind>,
}

impl<'a> Resolver<'a> {
    pub fn new(decls: &'a [Decl]) -> Self {
        Self {
            decls,
            symbols: HashMap::new(),
        }
    }

    pub fn resolve(mut self) -> Result<(), Error> {
        self.collect_symbols()?;
        self.check_field_types()?;
        Ok(())
    }

    fn collect_symbols(&mut self) -> Result<(), Error> {
        for decl in self.decls {
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

    fn check_field_types(&self) -> Result<(), Error> {
        for decl in self.decls {
            let Decl::Type(type_decl) = decl else {
                continue;
            };

            for field in type_decl.fields() {
                if let Type::Custom(name) = field.ty() {
                    if !self.symbols.contains_key(name) {
                        return Err(Error::UnknownType {
                            name: name.clone(),
                            used_in: format!("{}.{}", type_decl.name(), field.name()),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexing::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_forward_reference_resolves() {
        let source = "
            type User {
                name string,
                lifeState HumanLifeState
            }

            enum HumanLifeState {
                Alive,
                Dead
            }
        ";
        let tokens = Lexer::new(source).get_tokens().unwrap();
        let decls = Parser::new(tokens).parse().unwrap();

        assert!(Resolver::new(&decls).resolve().is_ok());
    }

    #[test]
    fn unknown_type_is_rejected() {
        let source = "type User { pet Dog }";
        let tokens = Lexer::new(source).get_tokens().unwrap();
        let decls = Parser::new(tokens).parse().unwrap();

        assert!(matches!(
            Resolver::new(&decls).resolve(),
            Err(Error::UnknownType { .. })
        ));
    }
}
