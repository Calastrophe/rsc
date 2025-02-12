use crate::{debugger::Debugger, emulator::Assembler};

#[derive(Default)]
pub struct BytecodeViewer {}

impl BytecodeViewer {
    fn name(&self) -> &'static str {
        "BytecodeViewer"
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
