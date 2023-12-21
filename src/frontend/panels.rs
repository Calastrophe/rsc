use super::{file::FileDialog, InterfaceState, Panel, View};
use crate::emulator::{assembler::Assembler, Emulator};

#[derive(Default)]
pub enum SelPanel {
    #[default]
    CPU, // Memory view with current register state, can easily modify instructions.
    Symbols,     // All of the symbols and their current values, easily able to modify.
    Breakpoints, // All of the breakpoints, disabled or enabled, easily able to modify.
}

#[derive(Default)]
pub struct Breakpoints;

impl Panel for Breakpoints {
    fn name(&self) -> &'static str {
        "breakpoints"
    }

    fn show(&mut self, ctx: &egui::Context, interface: &mut InterfaceState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ctx, interface, ui);
        });
    }
}

impl View for Breakpoints {
    fn ui(&mut self, ctx: &egui::Context, interface: &mut InterfaceState, ui: &mut egui::Ui) {
        todo!()
    }
}

#[derive(Default)]
pub struct Symbols;

impl Panel for Symbols {
    fn name(&self) -> &'static str {
        "symbols"
    }

    fn show(&mut self, ctx: &egui::Context, interface: &mut InterfaceState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ctx, interface, ui);
        });
    }
}

impl View for Symbols {
    fn ui(&mut self, ctx: &egui::Context, interface: &mut InterfaceState, ui: &mut egui::Ui) {
        todo!()
    }
}

#[derive(Default)]
pub struct CPU;

impl Panel for CPU {
    fn name(&self) -> &'static str {
        "cpu"
    }

    fn show(&mut self, ctx: &egui::Context, interface: &mut InterfaceState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ctx, interface, ui);
        });
    }
}

impl View for CPU {
    fn ui(&mut self, ctx: &egui::Context, interface: &mut InterfaceState, ui: &mut egui::Ui) {
        todo!()
    }
}

#[derive(Default)]
pub struct TopState {
    pub fd: FileDialog,
    input: String,
    step_scale: u8,
}

#[derive(Default)]
pub struct Top;

impl Panel for Top {
    fn name(&self) -> &'static str {
        "top_bar"
    }

    fn show(&mut self, ctx: &egui::Context, interface: &mut InterfaceState) {
        egui::TopBottomPanel::top(self.name()).show(ctx, |ui| {
            self.ui(ctx, interface, ui);
        });
    }
}

impl View for Top {
    fn ui(&mut self, ctx: &egui::Context, interface: &mut InterfaceState, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                if ui.button("Open file").clicked() {
                    interface.top.fd.open()
                }
            });

            if let Some(emulator) = &mut interface.emulator {
                ui.menu_button("Debug", |ui| {
                    if ui.button("Add breakpoint").clicked() {
                        // Open the modal
                    }
                    if ui.button("Remove breakpoint").clicked() {
                        // Open the modal
                    }
                });
            }
        });

        egui::menu::bar(ui, |ui| {
            // Draw the buttons needed for pausing and resuming, running till next breakpoint and
            // stepping over.
        });
    }
}
