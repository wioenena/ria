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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[TypeDeclField] {
        self.fields.as_slice()
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}
