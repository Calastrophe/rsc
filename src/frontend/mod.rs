use crate::emulator::Emulator;
use egui_modal::Modal;
use file::FileDialog;
mod file;

// The goal is to have something similar to x64dbg type menu

#[derive(Default)]
pub struct Interface {
    fd: FileDialog,
    emulator: Option<Emulator>,
    step_scale: usize,
}

impl Interface {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Interface {
            step_scale: 1,
            ..Default::default()
        }
    }

    pub fn file_menu(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            if ui.button("Open file").clicked() {
                self.fd.open()
            }
        });
    }

    pub fn debugger_menu(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if let Some(emulator) = &self.emulator {
            // let modal = Modal::new(ctx, "debug_modal");

            ui.menu_button("Debug", |ui| {
                if ui.button("Add breakpoint").clicked() {
                    // Open the modal
                }
            });
        }
    }

    pub fn register_state(&mut self, ui: &mut egui::Ui) {}

    pub fn memory_view(&mut self, ui: &mut egui::Ui) {}
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file_menu(ctx, ui);
                self.debugger_menu(ctx, ui);
            });
        });

        // egui::SidePanel::left("register_state").show(ctx, |ui| {});

        // egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
