use eframe::egui;

mod file_tree_panel;
mod info_panel;
mod top_panel;
pub use file_tree_panel::FileTreePanel;
pub use info_panel::InfoPanel;
pub use top_panel::TopPanel;

pub trait Panel {
  fn draw(&mut self, ctx: &egui::Context);
}
