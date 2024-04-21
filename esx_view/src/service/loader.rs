use std::{fs::File, path::PathBuf, rc::Rc};

use crate::{files::ESxFile, Result};
use esx_lib::esx::ESx;

#[derive(Debug, Default)]
pub struct ESxLoader {
  esx_list: Vec<Rc<ESxFile>>,
}

impl ESxLoader {
  pub fn esx_list(&self) -> &Vec<Rc<ESxFile>> {
    &self.esx_list
  }
  pub fn esx_list_mut(&mut self) -> &mut Vec<Rc<ESxFile>> {
    &mut self.esx_list
  }
  pub fn load_file(&mut self, path: &PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let mut esx = ESx::from_file(&file)?;
    esx.process();
    let file = Rc::new(ESxFile::new(path.clone(), esx));
    self.esx_list.push(file);
    Ok(())
  }
}
