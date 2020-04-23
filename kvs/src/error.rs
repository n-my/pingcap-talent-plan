use failure::Fail;
use std::io;
use std::result;

/// Doc
pub type Result<T> = result::Result<T, KvsError>;

/// Doc
#[derive(Fail, Debug)]
pub enum KvsError {
    /// Doc
    #[fail(display = "Input was invalid UTF-8 at index {}", _0)]
    Utf8Error(usize),
    /// Doc
    #[fail(display = "Key not found")]
    KeyNotFound(),
    /// Doc
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Docs
    #[fail(display = "{}", _0)]
    SerdeJson(#[cause] serde_json::error::Error),
}

impl From<io::Error> for KvsError {
    fn from(io_error: io::Error) -> KvsError {
        KvsError::Io(io_error)
    }
}
impl From<serde_json::error::Error> for KvsError {
    fn from(serde_json_error: serde_json::error::Error) -> KvsError {
        KvsError::SerdeJson(serde_json_error)
    }
}
