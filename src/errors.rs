use std::fmt::{self, Debug};

pub type DecodeResult<T> = std::result::Result<T, FailedDecode>;

#[derive(Debug)]
pub struct FailedDecode {
    pub bytes: u64,
}

impl fmt::Display for FailedDecode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed decode starting from byte {:X}", self.bytes)
    }
}

impl std::error::Error for FailedDecode {}
