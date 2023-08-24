
use crate::{emulator::Emulator, util::{types::Register, Memory}, file::FileDialog, parser::Assembler};

#[derive(Default)]
pub struct GUI {
    fd: FileDialog,
    file_contents: String,
    emulator: Option<Emulator>,
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for GUI {
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 _frame.close();
        //             }
        //         });
        //     });
        // });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.heading("Registers");
            
            for register in Register::iter() {
                if let Some(emulator) = &self.emulator {
                    ui.label(format!("{} : {}", register.as_str(), emulator.registers.get(*register)));
                } else {
                    ui.label(format!("{} : {}", register.as_str(), 0));
                }
            }    
            if let Some(emulator) = &mut self.emulator {
                if ui.button("Step forward").clicked() {
                    emulator.stepi(1)
                }
                if ui.button("Step backward").clicked() {
                    emulator.backi(1)
                }
            } else {
                if ui.button("Open file").clicked() {
                    self.fd.open()
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {

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

