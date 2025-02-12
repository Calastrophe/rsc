mod bytecode_viewer;
mod cpu;
mod editor;
mod top;
mod variable_viewer;
use egui::{TopBottomPanel, Ui};
use crate::debugger::Debugger;

#[macro_export]
macro_rules! impl_component_name {
    // Remove the type parameter and use impl_type instead
    () => {
        fn name(&self) -> &'static str {
            stringify!(Self)
        }
    };
}





pub trait Component { 
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut Ui);
}


#[derive(Default)]
pub struct Interface {
    pub debugger: Option<Debugger>,
}

impl Interface {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Interface {
            ..Default::default()
        }
    }
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel(ui, |ui| { 
            ui.button("Test");
        })





    }
}
