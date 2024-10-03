// use crate::domain::foo::FooId;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ChecksumError {
    address: String,
}

impl ChecksumError {
    pub fn new(address: &str) -> Self {
        ChecksumError {
            address: address.to_string(),
        }
    }
}

impl Error for ChecksumError {}

impl Display for ChecksumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid checksum for [{:?}].", self.address)
    }
}
