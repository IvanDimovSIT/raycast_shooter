use macroquad::input::{get_keys_down, get_keys_pressed, get_keys_released, KeyCode};

#[derive(Debug, PartialEq)]
pub enum Operation {
    Left,
    Right,
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    StartShooting,
    StopShooting,
}

pub fn get_input(_screen_size: (f32, f32)) -> Vec<Operation> {
    let pressed = get_keys_pressed().into_iter().filter_map(|key| match key {
        KeyCode::Space => Some(Operation::StartShooting),
        _ => None,
    });

    let released = get_keys_released().into_iter().filter_map(|key| match key {
        KeyCode::Space => Some(Operation::StopShooting),
        _ => None,
    });

    get_keys_down()
        .into_iter()
        .filter_map(|key| match key {
            KeyCode::A | KeyCode::Left => Some(Operation::Left),
            KeyCode::D | KeyCode::Right => Some(Operation::Right),
            KeyCode::W | KeyCode::Up => Some(Operation::Forward),
            KeyCode::S | KeyCode::Down => Some(Operation::Back),
            KeyCode::Q => Some(Operation::StrafeLeft),
            KeyCode::E => Some(Operation::StrafeRight),
            _ => None,
        })
        .chain(pressed)
        .chain(released)
        .collect()
}
