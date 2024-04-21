use eframe::egui::{self, CentralPanel, ScrollArea};
use egui_extras::{Column, TableBuilder};

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
    let file = match self.service.get_active_file() {
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
            ui.label(format!("Record Count: {}", record_count));
            let mut table = TableBuilder::new(ui);
            table = table.resizable(true).auto_shrink(false);
            table = table.sense(egui::Sense::click());
            table = table
              .column(Column::initial(80.0).range(20.0..=200.0).clip(true))
              .column(Column::remainder());
            let header = |mut header: egui_extras::TableRow<'_, '_>| {
              header.col(|ui| {
                ui.strong("Group");
              });
              header.col(|ui| {
                ui.strong("Record Count");
              });
            };
            let body = |mut body: egui_extras::TableBody<'_>| {
              for group in file.get_top_groups() {
                body.row(20.0, |mut row| {
                  row.col(|ui| {
                    ui.label(group.get_label().to_string());
                  });
                  row.col(|ui| {
                    ui.label(group.get_data().get_records_recurse().len().to_string());
                  });
                });
              }
            };
            table.header(20.0, header).body(body);
          }
          None => {
            ui.heading("Welcome to ESX View");
            ui.label("This is a simple app to view ESX data.");
          }
        });
    });
  }
}
