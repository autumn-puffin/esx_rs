#[derive(Debug)]
pub enum Error {
  #[deprecated = "Use specific error types instead"]
  Undefined,
  Io(std::io::Error),
  Ron(ron::Error),
  SpannedRon(ron::error::SpannedError),
  Esx(esx_lib::Error),
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::Io(err)
  }
}
impl From<ron::Error> for Error {
  fn from(err: ron::Error) -> Self {
    Error::Ron(err)
  }
}
impl From<ron::error::SpannedError> for Error {
  fn from(err: ron::error::SpannedError) -> Self {
    Error::SpannedRon(err)
  }
}
impl From<esx_lib::Error> for Error {
  fn from(err: esx_lib::Error) -> Self {
    Error::Esx(err)
  }
}

pub type Result<T> = std::result::Result<T, Error>;
