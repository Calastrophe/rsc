use crate::debugger::Debugger;

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
        todo!()
    }
}
