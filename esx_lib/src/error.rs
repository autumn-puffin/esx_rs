use flate2::DecompressError;
use std::array::TryFromSliceError;
use std::io::Error as IoError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  #[deprecated = "Errors should be defined, only use for development"]
  Undefined,
  IoError(IoError),
  TryFromSliceError(TryFromSliceError),
  DecompressError(DecompressError),
  NonGroupSignature(Vec<u8>),
  BufferTooShort,
  TES3Header,
  UnknownFileType,
  UnknownGroupLabelType(u32),
}
impl From<IoError> for Error {
  fn from(e: IoError) -> Self {
    Error::IoError(e)
  }
}
impl From<TryFromSliceError> for Error {
  fn from(e: TryFromSliceError) -> Self {
    Error::TryFromSliceError(e)
  }
}
impl From<DecompressError> for Error {
  fn from(e: DecompressError) -> Self {
    Error::DecompressError(e)
  }
}
