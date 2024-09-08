use macroquad::{
    input::{is_key_pressed, set_cursor_grab, show_mouse},
    miniquad::window::set_fullscreen,
};

use crate::constants::FOCUS_KEY;

pub struct FocusManager {
    is_focused: bool,
}
impl FocusManager {
    pub fn new() -> Self {
        set_cursor_grab(true);
        show_mouse(false);
        set_fullscreen(true);
        Self { is_focused: true }
    }

    pub fn update(&mut self) {
        if is_key_pressed(FOCUS_KEY) {
            self.is_focused = !self.is_focused;
            set_cursor_grab(self.is_focused);
            show_mouse(!self.is_focused);
            set_fullscreen(self.is_focused);
        }
    }
}
impl Drop for FocusManager {
    fn drop(&mut self) {
        set_cursor_grab(false);
        show_mouse(true);
        set_fullscreen(false);
    }
}
