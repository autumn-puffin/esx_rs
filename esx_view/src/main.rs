mod app;
mod error;
mod files;
mod panels;
mod service;

pub use error::{Error, Result};

use app::ESXView;

fn main() {
  let eframe_options = eframe::NativeOptions::default();
  let app = Box::<ESXView>::default();
  let _ = eframe::run_native("ESX View", eframe_options, Box::new(|_cc| app));
}
