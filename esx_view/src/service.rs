use std::{
  path::PathBuf,
  rc::Rc,
  sync::{Arc, Mutex},
};

use self::loader::ESxLoader;
use crate::{files::ESxFile, Result};

pub mod loader;

#[derive(Debug, Default)]
pub struct ESXService {
  loader: ESxLoader,
  active_file: Option<usize>,
}
impl ESXService {
  pub fn loader(&self) -> &ESxLoader {
    &self.loader
  }
}

#[derive(Debug, Default, Clone)]
pub struct ServiceController {
  pub esx_service: Arc<Mutex<ESXService>>,
}

impl ServiceController {
  fn lock_esx(&self) -> std::sync::MutexGuard<'_, ESXService> {
    self.esx_service.lock().unwrap()
  }
}

impl ServiceController {
  pub fn load_esx_file(&self, path: &PathBuf) -> Result<()> {
    self.lock_esx().loader.load_file(path)
  }
  pub fn drop_esx_file(&self, index: usize) -> Result<()> {
    if index >= self.lock_esx().loader.esx_list().len() {
      return Err("Index out of bounds".into());
    }
    self.lock_esx().loader.esx_list_mut().remove(index);
    Ok(())
  }
  pub fn get_esx_file(&self, index: usize) -> Option<Rc<ESxFile>> {
    self.lock_esx().loader.esx_list().get(index).cloned()
  }
  pub fn get_esx_list(&self) -> Vec<Rc<ESxFile>> {
    self.lock_esx().loader.esx_list().clone()
  }

  pub fn set_active_file(&self, index: usize) {
    self.lock_esx().active_file = Some(index);
  }
  pub fn get_active_file(&self) -> Option<usize> {
    self.lock_esx().active_file
  }
  pub fn clear_active_file(&self) {
    self.lock_esx().active_file = None;
  }
}
