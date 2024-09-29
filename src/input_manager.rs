use crate::math::Vector2;
use wasm4 as w4;

const SINGLETON: InputManager = InputManager{};
static mut PREVIOUS_GAMEPAD: u8 = 0;

#[derive(Default)]
pub struct InputManager;


impl InputManager {
    pub fn get_singleton() -> Self {
        return SINGLETON
    }

    pub fn get_gamepad(&self) -> u8 {
        return unsafe { *w4::sys::GAMEPAD1 }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        return self.get_gamepad() & key != 0
    }

    pub fn is_key_released(&self, key: u8) -> bool {
        return self.get_gamepad() & key == 0
    }

    pub fn get_vec(&self, positive_x: u8, positive_y: u8, negative_x: u8, negative_y: u8) -> Vector2 {
        let mut vec: Vector2 = Vector2::default();
        if self.is_key_pressed(positive_x) {
            vec.0 += 1;
        }
        if self.is_key_pressed(positive_y) {
            vec.1 += 1;
        }
        if self.is_key_pressed(negative_x) {
            vec.0 -= 1;
        }
        if self.is_key_pressed(negative_y) {
            vec.1 -= 1;
        }
        return vec
    }

    pub fn get_axis(&self, positive: u8, negative: u8) -> i8 {
        if self.is_key_pressed(positive) {
            return 1
        }
        if self.is_key_pressed(negative) {
            return -1
        }
        return 0
    }

    pub fn is_key_just_pressed(&self, key: u8) -> bool {
        let was_pressed_this_frame = unsafe {
            let previous = PREVIOUS_GAMEPAD;
            let gamepad = self.get_gamepad();
            let pressed_this_frame = gamepad & (gamepad ^ previous);
            PREVIOUS_GAMEPAD = gamepad;
            pressed_this_frame
        };

        return was_pressed_this_frame & key != 0
    }

    pub fn is_key_just_released(&self, key: u8) -> bool {
        unsafe {
            if PREVIOUS_GAMEPAD & key != 0 && self.get_gamepad() & key == 0 {
                return true
            }
        }
        return false
    }
}