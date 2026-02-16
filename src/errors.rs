use std::fmt::{self, Debug, Error};

pub type DecodeResult<T> = std::result::Result<T, FailedDecode>;

#[derive(Debug)]
pub struct FailedDecode {
    pub byte1: u8,
}

impl fmt::Display for FailedDecode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed decode starting from byte {:X}", self.byte1)
    }
}

impl std::error::Error for FailedDecode {}
