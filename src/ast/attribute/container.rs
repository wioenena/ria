use std::collections::HashMap;

use super::{AttributeField, AttributeKey, AttributeTargetLanguage};

#[derive(Debug, PartialEq, Default)]
pub struct AttributeContainer {
    attributes: HashMap<AttributeTargetLanguage, HashMap<AttributeKey, AttributeField>>,
}

impl AttributeContainer {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn add_field(
        &mut self,
        target: AttributeTargetLanguage,
        key: AttributeKey,
        field: AttributeField,
    ) {
        self.attributes
            .entry(target)
            .or_default()
            .insert(key, field);
    }

    pub fn get_attributes(
        &self,
    ) -> &HashMap<AttributeTargetLanguage, HashMap<AttributeKey, AttributeField>> {
        &self.attributes
    }
}
