#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

impl Boss {
    pub fn new(name: &str, age: u8) -> Boss {
        Boss { name:name.to_string(), age }
    }
}
