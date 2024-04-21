pub mod error;
pub mod statistics;

pub use error::{Error, Result};
pub mod load {
  use std::fs::File;
  use std::path::Path;

  use crate::error::Result;
  use esx_lib::esx::ESx;

  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ESx> {
    let file = File::open(path)?;
    let esx = ESx::from_file(&file)?;

    Ok(esx)
  }

  pub fn from_ron_file<P: AsRef<Path>>(path: P) -> Result<ESx> {
    let file = File::open(path)?;
    let esx: ESx = ron::de::from_reader(file)?;

    Ok(esx)
  }
}

pub mod save {
  use std::fs::File;
  use std::path::Path;

  use crate::error::Result;
  use esx_lib::esx::ESx;
  use ron::ser::PrettyConfig;

  pub fn to_ron_file<P: AsRef<Path>>(esx: &ESx, path: P) -> Result<()> {
    let file = File::create(path)?;
    ron::ser::to_writer_pretty(file, esx, PrettyConfig::default())?;

    Ok(())
  }
}
