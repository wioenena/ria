#[derive(Debug, PartialEq)]
pub struct EnumDecl {
    name: String,
    values: Vec<String>,
}

impl EnumDecl {
    pub fn new(name: String, values: Vec<String>) -> Self {
        Self { name, values }
    }
}
