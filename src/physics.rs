use crate::game_world::Component;
use crate::math::Vector2;

pub const GRAVITY: f32 = 9.8;

pub struct CollisionShape {
    _relative_position: Vector2,
    _size: Vector2
}

impl Component for CollisionShape {
    fn get_name(&self) -> &str {
        return "CollisionShape"
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