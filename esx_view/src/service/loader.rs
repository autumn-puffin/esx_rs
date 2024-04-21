use std::{fs::File, path::PathBuf};

use crate::{files::ESxFile, Result};
use esx_lib::esx::ESx;

#[derive(Debug, Default)]
pub struct ESxLoader {
  esx_list: Vec<ESxFile>,
}

impl ESxLoader {
  pub fn esx_list(&self) -> &Vec<ESxFile> {
    &self.esx_list
  }
  pub fn esx_list_mut(&mut self) -> &mut Vec<ESxFile> {
    &mut self.esx_list
  }
  pub fn load_file(&mut self, path: &PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let esx = ESx::from_file(&file)?;
    let file = ESxFile::new(path.clone(), esx);
    self.esx_list.push(file);
    Ok(())
  }
}
