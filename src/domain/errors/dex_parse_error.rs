// use crate::domain::foo::FooId;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct DexParseError {
    name: String,
}

impl DexParseError {
    pub fn new(name: &str) -> Self {
        DexParseError {
            name: name.to_string(),
        }
    }
}

impl Error for DexParseError {}

impl Display for DexParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo with id [{:?}] not found.", self.name)
    }
}
