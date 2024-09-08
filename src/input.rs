use std::collections::HashSet;

use core::hash::Hash;
use macroquad::input::{
    get_keys_down, is_mouse_button_down, mouse_delta_position, KeyCode, MouseButton,
};
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::iter::once;

use crate::constants::KEYS_TURN_SPEED;
use crate::constants::MOUSE_TURN_SPEED;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Left(f32),
    Right(f32),
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Shoot,
}
impl Hash for Operation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
impl Eq for Operation {}

fn get_unique_input(input: HashSet<Operation>) -> Vec<Operation> {
    input.into_iter().collect()
}

fn get_mouse_input() -> Vec<Operation> {
    (vec![])
        .into_iter()
        .chain(once(if is_mouse_button_down(MouseButton::Left) {
            Some(Operation::Shoot)
        } else {
            None
        }))
        .chain(once(if mouse_delta_position().x < -f32::EPSILON {
            Some(Operation::Right(
                -mouse_delta_position().x * MOUSE_TURN_SPEED,
            ))
        } else if mouse_delta_position().x > f32::EPSILON {
            Some(Operation::Left(mouse_delta_position().x * MOUSE_TURN_SPEED))
        } else {
            None
        }))
        .filter_map(|op| op)
        .collect()
}

pub fn get_input() -> Vec<Operation> {
    let input = get_keys_down()
        .into_iter()
        .filter_map(|key| match key {
            KeyCode::Q | KeyCode::Left => Some(Operation::Left(KEYS_TURN_SPEED)),
            KeyCode::E | KeyCode::Right => Some(Operation::Right(KEYS_TURN_SPEED)),
            KeyCode::W | KeyCode::Up => Some(Operation::Forward),
            KeyCode::S | KeyCode::Down => Some(Operation::Back),
            KeyCode::A => Some(Operation::StrafeLeft),
            KeyCode::D => Some(Operation::StrafeRight),
            KeyCode::Space => Some(Operation::Shoot),
            _ => None,
        })
        .chain(get_mouse_input())
        .collect();

    get_unique_input(input)
}
