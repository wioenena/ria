use crate::ast::Value;

use super::{AttributeKey, AttributeUsedIn};

#[derive(Debug, PartialEq)]
pub struct AttributeField {
    key: AttributeKey,
    value: Value,
    used_in: AttributeUsedIn,
    line: usize,
    column: usize,
}

impl AttributeField {
    pub fn new(
        key: AttributeKey,
        value: Value,
        used_in: AttributeUsedIn,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            key,
            value,
            used_in,
            line,
            column,
        }
    }

    pub fn key(&self) -> AttributeKey {
        self.key
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn used_in(&self) -> AttributeUsedIn {
        self.used_in
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
