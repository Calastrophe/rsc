use crate::{debugger::Debugger, emulator::Assembler};

#[derive(Default)]
pub struct Top {}

impl Top {
    fn name(&self) -> &'static str {
        "Top"
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        _debugger: &mut Option<Debugger>,
        _assembler: &mut Option<Assembler>,
    ) {
        ui.label(self.name());
    }
}
