use crate::math::Vector2;
use wasm4 as w4;

pub struct Renderer {}

impl Renderer {
    pub fn render(data: &[u8], position: Vector2, size: Vector2) {
        unsafe {
            w4::sys::blit(data.as_ptr(), position.0, position.1, size.0 as u32, size.1 as u32, w4::sys::BLIT_1BPP);   
        }
    }
}