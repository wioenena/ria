use crate::types::Type;

use super::attribute::AttributeContainer;

#[derive(Debug, PartialEq)]
pub struct TypeDeclField {
    name: String,
    ty: Type,
    attribute_container: AttributeContainer,
    line: usize,
    column: usize,
}

impl TypeDeclField {
    pub fn new(
        name: String,
        ty: Type,
        container: AttributeContainer,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            name,
            ty,
            attribute_container: container,
            line,
            column,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn attribute_container(&self) -> &AttributeContainer {
        &self.attribute_container
    }

    pub fn attribute_container_mut(&mut self) -> &mut AttributeContainer {
        &mut self.attribute_container
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
