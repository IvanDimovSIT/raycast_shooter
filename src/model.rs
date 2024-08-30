use std::{f32::consts::PI, fmt::Debug, time::Duration};

use key_object::KeyObject;
use macroquad::math::Vec2;
use uuid::Uuid;

pub mod key_object;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Texture {
    Debug,
    Stone,
    Key1,
    Key2,
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub position: Vec2,
    pub size: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub entity: Entity,
    pub look: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct Wall {
    pub texture: Texture,
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    PickUpKey(Uuid),
}

pub struct GameObjects {
    pub player: Player,
    pub walls: Vec<Wall>,
    pub keys: Vec<KeyObject>,
}
