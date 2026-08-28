#[derive(Debug, PartialEq)]
pub struct EnumDecl {
    name: String,
    values: Vec<String>,
}

impl EnumDecl {
    pub fn new(name: String, values: Vec<String>) -> Self {
        Self { name, values }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values(&self) -> &[String] {
        self.values.as_slice()
    }
}
