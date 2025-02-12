use crate::impl_component_name;
use super::Component;




struct Editor { 

}


impl Component for Editor {
    impl_component_name!();

    
    fn show(&mut self, ui: &mut egui::Ui) {
        todo!()
    }
}