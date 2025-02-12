use crate::impl_component_name;
use super::Component;




struct VariableViewer { 

}


impl Component for VariableViewer {
    impl_component_name!();

    
    fn show(&mut self, ui: &mut egui::Ui) {
        todo!()
    }
}