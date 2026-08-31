use super::Decl;
use super::attribute::AttributeContainer;

#[derive(Debug)]
pub struct Program {
    decls: Vec<Decl>,
    attribute_container: AttributeContainer,
}

impl Program {
    pub fn new(decls: Vec<Decl>) -> Self {
        Self {
            decls,
            attribute_container: AttributeContainer::new(),
        }
    }

    pub fn add_decl(&mut self, decl: Decl) {
        self.decls.push(decl);
    }

    pub fn decls(&self) -> &[Decl] {
        &self.decls
    }

    pub fn attribute_container(&self) -> &AttributeContainer {
        &self.attribute_container
    }

    pub fn attribute_container_mut(&mut self) -> &mut AttributeContainer {
        &mut self.attribute_container
    }
}
