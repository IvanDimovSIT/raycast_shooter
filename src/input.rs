use macroquad::input::{get_keys_down, get_keys_pressed, get_keys_released, KeyCode};

#[derive(Debug, PartialEq)]
pub enum Operation {
    Left,
    Right,
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Shoot,
}

pub fn get_input(_screen_size: (f32, f32)) -> Vec<Operation> {
    get_keys_down()
        .into_iter()
        .filter_map(|key| match key {
            KeyCode::A | KeyCode::Left => Some(Operation::Left),
            KeyCode::D | KeyCode::Right => Some(Operation::Right),
            KeyCode::W | KeyCode::Up => Some(Operation::Forward),
            KeyCode::S | KeyCode::Down => Some(Operation::Back),
            KeyCode::Q => Some(Operation::StrafeLeft),
            KeyCode::E => Some(Operation::StrafeRight),
            KeyCode::Space => Some(Operation::Shoot),
            _ => None,
        })
        .collect()
}
