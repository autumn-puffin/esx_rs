use eframe::egui::TopBottomPanel;
use rfd::FileDialog;

use crate::service::ServiceController;

#[derive(Default)]
pub struct TopPanel {
  service: ServiceController,
}
impl TopPanel {
  pub fn new(service: ServiceController) -> TopPanel {
    TopPanel { service }
  }
}

impl super::Panel for TopPanel {
  #[inline]
  fn draw(&mut self, ctx: &eframe::egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
      ui.menu_button("Load...", |ui| {
        if ui.button("Load ESX").clicked() {
          if let Some(path) = FileDialog::new()
            .add_filter("esx", &["esp", "esl", "esm"])
            .pick_file()
          {
            let _ = self.service.load_esx_file(&path);
          }
          ui.close_menu()
        }
      });
    });
  }
}
