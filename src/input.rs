use macroquad::input::{get_keys_down, KeyCode};

#[derive(Debug)]
pub enum Operation {
    Left,
    Right,
    Forward,
    Back,
}

pub fn get_input(_screen_size: (f32, f32)) -> Vec<Operation> {
    get_keys_down()
        .into_iter()
        .filter_map(|key| match key {
            KeyCode::A | KeyCode::Left => Some(Operation::Left),
            KeyCode::D | KeyCode::Right => Some(Operation::Right),
            KeyCode::W | KeyCode::Up => Some(Operation::Forward),
            KeyCode::S | KeyCode::Down => Some(Operation::Back),
            _ => None,
        })
        .collect()
}
