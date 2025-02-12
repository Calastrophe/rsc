use super::Component;

#[derive(Default)]
pub struct CpuState {}

impl Component for CpuState {
    fn name(&self) -> &'static str {
        "CpuState"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label(self.name());
    }
}
