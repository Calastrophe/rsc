use super::Component;

#[derive(Default)]
pub struct BytecodeViewer {}

impl Component for BytecodeViewer {
    fn name(&self) -> &'static str {
        "BytecodeViewer"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label(self.name());
    }
}
