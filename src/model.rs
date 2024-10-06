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
pub enum TextureId {
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
    MeleeSlowEnemy1,
    MeleeSlowEnemy2,
    MeleeSlowEnemy3,
    MeleeSlowEnemy4,
    MeleeSlowEnemy5,
    MeleeSlowEnemy6,
    MeleeSlowEnemy7,
    MeleeSlowEnemy8,
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
    TextFindTheKeys,
}
impl Default for TextureId {
    fn default() -> Self {
        Self::Debug
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Animation {
    Enemy,
    RangedEnemy,
    MeleeSlowEnemy,
    Key,
    Gun,
    Explosion,
}
impl Animation {
    pub fn get_textures(&self) -> Vec<TextureId> {
        match self {
            Animation::Enemy => vec![
                TextureId::Enemy1,
                TextureId::Enemy2,
                TextureId::Enemy3,
                TextureId::Enemy4,
                TextureId::Enemy5,
                TextureId::Enemy6,
                TextureId::Enemy7,
                TextureId::Enemy8,
            ],
            Animation::Key => vec![TextureId::Key1, TextureId::Key2],
            Animation::Gun => vec![
                TextureId::Gun1,
                TextureId::Gun2,
                TextureId::Gun3,
                TextureId::Gun4,
                TextureId::Gun5,
                TextureId::Gun6,
                TextureId::Gun7,
                TextureId::Gun8,
            ],
            Animation::Explosion => vec![
                TextureId::Explostion2,
                TextureId::Explostion3,
                TextureId::Explostion4,
                TextureId::Explostion5,
                TextureId::Explostion6,
                TextureId::Explostion7,
                TextureId::Explostion8,
                TextureId::Explostion9,
            ],
            Animation::RangedEnemy => vec![
                TextureId::RangedEnemy1,
                TextureId::RangedEnemy2,
                TextureId::RangedEnemy3,
                TextureId::RangedEnemy4,
                TextureId::RangedEnemy5,
                TextureId::RangedEnemy6,
                TextureId::RangedEnemy7,
                TextureId::RangedEnemy8,
            ],
            Animation::MeleeSlowEnemy => vec![
                TextureId::MeleeSlowEnemy1,
                TextureId::MeleeSlowEnemy2,
                TextureId::MeleeSlowEnemy3,
                TextureId::MeleeSlowEnemy4,
                TextureId::MeleeSlowEnemy5,
                TextureId::MeleeSlowEnemy6,
                TextureId::MeleeSlowEnemy7,
                TextureId::MeleeSlowEnemy8,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SoundId {
    PickUpKey,
    PlayerTakeDamage,
    ShotHit,
    Shooting,
    Lose,
    Escape,
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
    pub texture: TextureId,
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    PickUpKey,
    EnemyKilled {
        position: Vec2,
    },
    LocationShot {
        position: Vec2,
    },
    PlayerTakeDamage(f32),
    CreateProjectile {
        position: Vec2,
        direction: Vec2,
        damage: f32,
    },
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
