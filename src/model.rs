use std::fmt::Debug;

use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Texture{
    Debug,
    Stone
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub position: Vec2,
    pub size: f32
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
    pub end: Vec2
}


