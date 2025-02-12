use super::Component;

#[derive(Default)]
pub struct Top {}

impl Component for Top {
    fn name(&self) -> &'static str {
        "Top"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label(self.name());
    }
}
