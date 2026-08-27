use crate::types::Type;

#[derive(Debug, PartialEq)]
pub struct TypeDecl {
    name: String,
    fields: Vec<TypeDeclField>,
}

impl TypeDecl {
    pub fn new(name: String, fields: Vec<TypeDeclField>) -> Self {
        Self { name, fields }
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeDeclField {
    name: String,
    ty: Type,
}

impl TypeDeclField {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }
}
