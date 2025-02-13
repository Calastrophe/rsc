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
        debugger: &mut Option<Debugger>,
        
        assembler: &mut Option<Assembler>,
        code: &str,
    ) {


        ui.vertical_centered(|ui| { 
            ui.horizontal(|ui| { 
     
            if let Some(debugger) =  debugger {
                ui.button(""); // pause
                ui.button("▶️"); // 
                ui.add(
                    egui::Slider::new::<u32>(&mut debugger.instructions_per_second , 1..=1000)
                    .text("IPS")

                );
            } else { 
                if ui.button("Assemble").clicked() { 
                    assembler.replace(Assembler::parse(code.to_string()));
                }

            }

            });




        });
    }
}
