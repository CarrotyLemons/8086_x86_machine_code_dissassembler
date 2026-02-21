use std::fmt::{self, Debug};

pub type DecodeResult<'a, T> = std::result::Result<T, FailedDecode<'a>>;

#[derive(Debug)]
pub struct FailedDecode<'a> {
    pub bytes: u8,
    pub message: &'a str,
}

impl<'a> fmt::Display for FailedDecode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed decode starting from byte {:X} - {}",
            self.bytes, self.message
        )
    }
}

impl<'a> std::error::Error for FailedDecode<'a> {}
