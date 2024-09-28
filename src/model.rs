use std::{fmt::Debug, time::Duration};

use decoration::Decoration;
use enemy::Enemy;
use key_object::KeyObject;
use macroquad::math::Vec2;
use projectile::Projectile;
use serde::Deserialize;

use crate::constants::{MAX_BULLETS, PLAYER_MAX_HEALTH};

pub mod decoration;
pub mod enemy;
pub mod key_object;
pub mod projectile;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
pub enum Texture {
    Debug,
    Stone,
    Metal,
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
    RangedEnemy1,
    RangedEnemy2,
    RangedEnemy3,
    RangedEnemy4,
    RangedEnemy5,
    RangedEnemy6,
    RangedEnemy7,
    RangedEnemy8,
    Projectile,
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
pub enum Animation {
    Enemy,
    RangedEnemy,
    Key,
    Gun,
    Explosion,
}
impl Animation {
    pub fn get_textures(&self) -> Vec<Texture> {
        match self {
            Animation::Enemy => vec![
                Texture::Enemy1,
                Texture::Enemy2,
                Texture::Enemy3,
                Texture::Enemy4,
                Texture::Enemy5,
                Texture::Enemy6,
                Texture::Enemy7,
                Texture::Enemy8,
            ],
            Animation::Key => vec![Texture::Key1, Texture::Key2],
            Animation::Gun => vec![
                Texture::Gun1,
                Texture::Gun2,
                Texture::Gun3,
                Texture::Gun4,
                Texture::Gun5,
                Texture::Gun6,
                Texture::Gun7,
                Texture::Gun8,
            ],
            Animation::Explosion => vec![
                Texture::Explostion2,
                Texture::Explostion3,
                Texture::Explostion4,
                Texture::Explostion5,
                Texture::Explostion6,
                Texture::Explostion7,
                Texture::Explostion8,
                Texture::Explostion9,
            ],
            Animation::RangedEnemy => vec![
                Texture::RangedEnemy1,
                Texture::RangedEnemy2,
                Texture::RangedEnemy3,
                Texture::RangedEnemy4,
                Texture::RangedEnemy5,
                Texture::RangedEnemy6,
                Texture::RangedEnemy7,
                Texture::RangedEnemy8,
            ],
        }
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
    PickUpKey(u64),
    EnemyKilled { position: Vec2 },
    LocationShot { position: Vec2 },
    PlayerTakeDamage(f32),
    CreateProjectile { position: Vec2, direction: Vec2 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShootingStatus {
    Shooting,
    NotShooting,
    Reloading,
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub shooting_status: ShootingStatus,
    pub time_since_last_shot: f32,
    pub bullets: usize,
    pub picked_up_keys: usize,
    pub health: f32,
}
impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            shooting_status: ShootingStatus::NotShooting,
            picked_up_keys: 0,
            health: PLAYER_MAX_HEALTH,
            time_since_last_shot: 0.0,
            bullets: MAX_BULLETS,
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
    pub exit_triggers: Vec<Entity>,
    pub decorations: Vec<Decoration>,
    pub projectiles: Vec<Projectile>,
}
