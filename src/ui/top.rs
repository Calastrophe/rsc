use crate::impl_component_name;
use super::Component;




struct Top { 

}


impl Component for Top {
    impl_component_name!();

    
    fn show(&mut self, ui: &mut egui::Ui) {
        todo!()
    }
}