use crate::impl_component_name;
use super::Component;




struct Cpu { 

}


impl Component for Cpu {
    impl_component_name!();

    
    fn show(&mut self, ui: &mut egui::Ui) {
        todo!()
    }
}