use crate::emulator::{Assembler, Emulator};
use crate::frontend::{file::FileDialog, panels::*, Panel, View};

#[derive(Default)]
pub struct InterfaceState {
    pub emulator: Option<Emulator>,
    pub current_tab: SelPanel,
    pub top: TopState,
}

#[derive(Default)]
pub struct Interface {
    state: InterfaceState,
}

impl Interface {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Interface {
            ..Default::default()
        }
    }
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(contents) = self.state.top.fd.get() {
            match Assembler::parse(contents) {
                Ok(asm) => self.state.emulator = Some(Emulator::new(asm)),
                Err(e) => {
                    // Pop egui_modal dialog with error mesage
                }
            }
        }

        Top::default().show(ctx, &mut self.state);
    }
}
