use macroquad::input::is_key_down;

#[derive(Debug)]
pub enum Operation {
    Left,
    Right,
    Forward,
    Back,
}

pub fn get_input(_screen_size: (f32, f32)) -> Vec<Operation> {
    let mut input = vec![];

    if is_key_down(macroquad::input::KeyCode::A) || is_key_down(macroquad::input::KeyCode::Left) {
        input.push(Operation::Left);
    }
    if is_key_down(macroquad::input::KeyCode::D) || is_key_down(macroquad::input::KeyCode::Right) {
        input.push(Operation::Right);
    }
    if is_key_down(macroquad::input::KeyCode::W) || is_key_down(macroquad::input::KeyCode::Up) {
        input.push(Operation::Forward);
    }
    if is_key_down(macroquad::input::KeyCode::S) || is_key_down(macroquad::input::KeyCode::Down) {
        input.push(Operation::Back);
    }

    input
}
