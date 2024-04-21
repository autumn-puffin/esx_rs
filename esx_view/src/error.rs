use esx_lib::Error as ESXError;
use std::{cell::BorrowMutError, io::Error as IoError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  #[deprecated = "Errors should be defined, only use for development"]
  Undefined,
  Custom(String),
  ESXError(ESXError),
  IoError(std::io::Error),
  RefCellBorrowMutError(BorrowMutError),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Error")
  }
}

impl From<String> for Error {
  fn from(e: String) -> Self {
    Error::Custom(e)
  }
}
impl From<&str> for Error {
  fn from(e: &str) -> Self {
    Error::Custom(e.to_string())
  }
}

impl From<ESXError> for Error {
  fn from(e: ESXError) -> Self {
    Error::ESXError(e)
  }
}
impl From<IoError> for Error {
  fn from(e: IoError) -> Self {
    Error::IoError(e)
  }
}
impl From<BorrowMutError> for Error {
  fn from(e: BorrowMutError) -> Self {
    Error::RefCellBorrowMutError(e)
  }
}
