use std::{fmt::Debug, time::Duration};

use decoration::Decoration;
use enemy::Enemy;
use key_object::KeyObject;
use macroquad::math::Vec2;
use uuid::Uuid;

pub mod decoration;
pub mod enemy;
pub mod key_object;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Texture {
    Debug,
    Stone,
    Key1,
    Key2,
    Gun1,
    Gun2,
    Gun3,
    Gun4,
    Gun5,
    Gun6,
    Gun7,
    Gun8,
    Enemy1,
    Enemy2,
    Enemy3,
    Enemy4,
    Enemy5,
    Enemy6,
    Enemy7,
    Enemy8,
    Skull,
    Explostion1,

    Explostion2,

    Explostion3,

    Explostion4,

    Explostion5,

    Explostion6,

    Explostion7,
    Explostion8,
    Explostion9,
}
impl Default for Texture {
    fn default() -> Self {
        Self::Debug
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub position: Vec2,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub entity: Entity,
    pub look: Vec2,
}

#[derive(Debug, Clone)]
pub struct Wall {
    pub texture: Texture,
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    PickUpKey(Uuid),
    EnemyKilled { position: Vec2 },
    LocationShot { position: Vec2 },
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub is_shooting: bool,
    pub health: f32,
}
impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            is_shooting: false,
            health: 100.0,
        }
    }
}

#[derive(Debug)]
pub struct GameObjects {
    pub player: Player,
    pub player_info: PlayerInfo,
    pub walls: Vec<Wall>,
    pub enemies: Vec<Enemy>,
    pub keys: Vec<KeyObject>,
    pub decorations: Vec<Decoration>,
}
