use esx_lib::esx::ESx;
use std::{ops::Deref, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ESxFile {
  file_path: PathBuf,
  esx: Box<ESx>,
}

impl Deref for ESxFile {
  type Target = ESx;
  fn deref(&self) -> &Self::Target {
    &self.esx
  }
}

impl ESxFile {
  pub fn new(file_path: PathBuf, esx: ESx) -> Self {
    let esx = Box::new(esx);
    ESxFile { esx, file_path }
  }
}
impl ESxFile {
  pub fn file_path(&self) -> &PathBuf {
    &self.file_path
  }
  pub fn esx(&self) -> &ESx {
    &self.esx
  }
}
impl ESxFile {
  pub fn file_name(&self) -> String {
    self
      .file_path
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string()
  }
}
