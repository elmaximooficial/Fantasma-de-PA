use crate::game_world::Component;

pub struct Movement;

impl Component for Movement {
    fn get_name(&self) -> &str {
        return "Movement"
    }

    fn start(&mut self) {

    }

    fn update(&mut self) {
        
    }

    fn render(&mut self) {
        
    }

    fn is_renderable(&self) -> bool {
        return false
    }
}