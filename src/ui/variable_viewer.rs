use crate::{debugger::Debugger, emulator::Assembler};

#[derive(Default)]
pub struct VariableViewer {}

impl VariableViewer {
    fn name(&self) -> &'static str {
        "VariableViewer"
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        _debugger: &Option<Debugger>,
        _assembler: &Option<Assembler>,
    ) {
        ui.label(self.name());
    }
}
