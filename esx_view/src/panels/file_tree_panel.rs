use eframe::egui::{self, Response, ScrollArea, SidePanel};
use egui_extras::{Column, TableBuilder};

use crate::service::{self, ServiceController};

#[derive(Default)]
pub struct FileTreePanel {
  service: ServiceController,
}

impl FileTreePanel {
  pub fn toggle_row_selection(&mut self, row_index: usize, row_response: &Response) {
    if row_response.clicked() {
      let selection = self.service.get_active_file();
      if selection == Some(row_index) {
        println!("Deselected");
        self.service.clear_active_file();
      } else {
        println!("Selected row {}", row_index);
        self.service.set_active_file(row_index);
      }
    }
  }

  pub fn new(service: ServiceController) -> FileTreePanel {
    FileTreePanel { service }
  }
}

impl super::Panel for FileTreePanel {
  fn draw(&mut self, ctx: &egui::Context) {
    SidePanel::left("file_tree_panel").show(ctx, |ui| {
      ScrollArea::horizontal().auto_shrink(false).show(ui, |ui| {
        let mut table = TableBuilder::new(ui);

        table = table.resizable(true).auto_shrink(false);
        table = table.sense(egui::Sense::click());

        table = table
          .column(Column::initial(80.0).range(20.0..=200.0).clip(true))
          .column(Column::remainder());

        let header = |mut header: egui_extras::TableRow<'_, '_>| {
          header.col(|ui| {
            ui.strong("File");
          });
          header.col(|ui| {
            ui.strong("______");
          });
        };
        let body = |mut body: egui_extras::TableBody<'_>| {
          let service = self.service.clone();
          let selection = service.get_active_file();
          let lock = service.lock_esx();
          let files = lock.loader().esx_list();
          for (index, file) in files.iter().enumerate() {
            body.row(20.0, |mut row| {
              row.set_selected(Some(index) == selection);

              row.col(|ui| {
                ui.label(file.file_name());
              });
              row.col(|ui| {});

              if row.response().clicked() {
                self.toggle_row_selection(index, &row.response());
              }
            });
          }
        };

        table.header(20.0, header).body(body);
      });
    });
  }
}
