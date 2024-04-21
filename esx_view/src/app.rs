use eframe::{egui, App, Frame};

use crate::{
  panels::{FileTreePanel, InfoPanel, Panel, TopPanel},
  service::ServiceController,
};

pub struct ESXView {
  service: ServiceController,
  top_panel: TopPanel,
  file_tree_panel: FileTreePanel,
  info_panel: InfoPanel,
}

impl Default for ESXView {
  fn default() -> Self {
    let service = ServiceController::default();
    Self {
      service: service.clone(),
      top_panel: TopPanel::new(service.clone()),
      file_tree_panel: FileTreePanel::new(service.clone()),
      info_panel: InfoPanel::new(service.clone()),
    }
  }
}

impl App for ESXView {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
    ctx.style_mut(|style| {
      style.wrap = Some(false);
    });

    self.top_panel.draw(ctx);
    self.file_tree_panel.draw(ctx);
    self.info_panel.draw(ctx);

    let Self {
      file_tree_panel,
      info_panel,
      ..
    } = self;
  }
}
