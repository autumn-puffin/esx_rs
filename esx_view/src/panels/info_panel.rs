use eframe::egui::{CentralPanel, ScrollArea};

use crate::{files::ESxFile, service::ServiceController};

#[derive(Default)]
pub struct InfoPanel {
  service: ServiceController,
}
impl InfoPanel {
  pub fn new(service: ServiceController) -> InfoPanel {
    InfoPanel { service }
  }
}
impl super::Panel for InfoPanel {
  #[inline]
  fn draw(&mut self, ctx: &eframe::egui::Context) {
    let file: Option<ESxFile> = match self.service.get_active_file() {
      Some(index) => self.service.get_esx_file(index),
      None => None,
    };
    CentralPanel::default().show(ctx, |ui| {
      ScrollArea::both()
        .auto_shrink(false)
        .show(ui, |ui| match file {
          Some(file) => {
            let header = file.get_header_record();
            let form_version = header.get_form_version();
            let record_count = file.get_all_records().len();
            ui.heading(file.file_name());
            ui.label(format!("Form Version: {}", form_version));
            ui.label(format!("Records: {}", record_count));
          }
          None => {
            ui.heading("Welcome to ESX View");
            ui.label("This is a simple app to view ESX data.");
          }
        });
    });
  }
}
