use std::collections::HashMap;

use super::attribute::{AttributeField, AttributeTargetLanguage};

#[derive(Debug, PartialEq)]
pub struct EnumDecl {
    name: String,
    values: Vec<String>,
    attributes: HashMap<AttributeTargetLanguage, Vec<AttributeField>>,
    line: usize,
    column: usize,
}

impl EnumDecl {
    pub fn new(
        name: String,
        values: Vec<String>,
        attributes: HashMap<AttributeTargetLanguage, Vec<AttributeField>>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            name,
            values,
            attributes,
            line,
            column,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values(&self) -> &[String] {
        self.values.as_slice()
    }

    pub fn attributes(&self) -> &HashMap<AttributeTargetLanguage, Vec<AttributeField>> {
        &self.attributes
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
