use crate::{debugger::Debugger, emulator::util::Register};

#[derive(Default)]
pub struct CpuState {}

impl CpuState {
    fn name(&self) -> &'static str {
        "CpuState"
    }

    pub fn show(&mut self, ui: &mut egui::Ui, debugger: &Option<Debugger>) {
        for register in Register::iter() {
            ui.label(format!(
                "{} : {}",
                register.as_str(),
                debugger
                    .as_ref()
                    .and_then(|debugger| { Some(debugger.read_reg(*register)) })
                    .unwrap_or(0)
            ));
        }
    }
}
