pub use self::interface::Interface;
use self::interface::InterfaceState;

mod file;
mod interface;
mod panels;

pub trait View {
    fn ui(&mut self, ctx: &egui::Context, interface: &mut InterfaceState, ui: &mut egui::Ui);
}

pub trait Panel {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, interface: &mut InterfaceState);
}
