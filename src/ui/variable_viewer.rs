use super::Component;

#[derive(Default)]
pub struct VariableViewer {}

impl Component for VariableViewer {
    fn name(&self) -> &'static str {
        "VariableViewer"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label(self.name());
    }
}
