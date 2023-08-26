
use crate::{emulator::Emulator, util::{types::Register, Memory}, file::FileDialog, parser::Assembler};

#[derive(Default)]
pub struct GUI {
    fd: FileDialog,
    file_contents: String,
    step_scale: usize,
    emulator: Option<Emulator>,
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        GUI { step_scale: 1, ..Default::default() }
    }
}

impl eframe::App for GUI {
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open file").clicked() {
                        self.fd.open()
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit").clicked() {
                        _frame.close()
                    }
                })
            });
            
            if let Some(emulator) = &mut self.emulator {
                ui.heading("Registers");
                for register in Register::iter() {
                    ui.label(format!("{} : {}", register.as_str(), emulator.registers.get(*register)));
                }
                if ui.button("Start").clicked() {
                    emulator.start()
                }
                if ui.button("Step forward").clicked() {
                    emulator.stepi(self.step_scale)
                }
                if ui.button("Step backward").clicked() {
                    emulator.backi(self.step_scale)
                }
                // Temporary
                emulator.set_breakpoint(42);
                if emulator.query(emulator.registers.get(Register::PC)) {
                    if ui.button("Step over").clicked() {
                        emulator.step_over()
                    }
                }
            }
        });


        if let Some(file_contents) = self.fd.get() {
            self.file_contents = file_contents;
            let assembler = Assembler::parse(&self.file_contents);
            let memory = Memory::new(&assembler.instructions);
            self.emulator = Some(Emulator::new(assembler, memory));
        }

    }
}


// There should be a context menu where you can open and load a file.

// The loading of the file will parse the tokens and determine if there is a parsing error.

// It will populate an optional memory view table, that you can show with a click of a button.

