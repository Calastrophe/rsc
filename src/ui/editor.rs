use super::Component;

#[derive(Default)]
pub struct Editor {}

impl Component for Editor {
    fn name(&self) -> &'static str {
        "Editor"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label(self.name());
    }
}
