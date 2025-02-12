use crate::debugger::Debugger;

#[derive(Default)]
pub struct CpuState {}

impl CpuState {
    fn name(&self) -> &'static str {
        "CpuState"
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _debugger: &Option<Debugger>) {
        ui.label(self.name());
    }
}
