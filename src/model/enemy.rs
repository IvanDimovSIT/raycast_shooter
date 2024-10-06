use std::time::Duration;

use crate::{
    constants::{
        ENEMY_BASE_HEIGHT_OFFSET, ENEMY_HEIGHT_AMPLITUDE, ENEMY_HEIGHT_CHANGE_ANIMATION_SPEED,
        MELEE_ENEMY_ATTACK_DELAY, MELEE_ENEMY_ATTACK_RANGE, MELEE_ENEMY_DAMAGE,
        MELEE_ENEMY_MOVE_SPEED, MELEE_SLOW_ENEMY_ANIMATION_SPEED, MELEE_SLOW_ENEMY_ATTACK_DELAY,
        MELEE_SLOW_ENEMY_ATTACK_RANGE, MELEE_SLOW_ENEMY_DAMAGE, MELEE_SLOW_ENEMY_HEALTH,
        MELEE_SLOW_ENEMY_MOVE_SPEED, RANGED_ENEMY_ANIMATION_SPEED, RANGED_ENEMY_ATTACK_DELAY,
        RANGED_ENEMY_DAMAGE, RANGED_ENEMY_MOVE_SPEED, RANGED_ENEMY_SHOOT_RANGE,
    },
    model::Animation,
    service::id_generator::generate_id,
};
use macroquad::math::{vec2, Vec2};
use serde::Deserialize;

use crate::{
    constants::{ENEMY_HP, ENEMY_SIZE, MELEE_ENEMY_ANIMATION_SPEED},
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
};

use super::{Entity, TextureId};

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum EnemyType {
    Melee,
    MeleeSlow,
    Ranged,
}
impl EnemyType {
    pub fn to_enemy(self, position: Vec2) -> Enemy {
        let id = generate_id();
        let entity = Entity {
            position,
            size: ENEMY_SIZE,
        };

        match self {
            EnemyType::Melee => Enemy {
                id,
                entity,
                hp: ENEMY_HP,
                attack_delay: 0.0,
                enemy_type: self,
            },
            EnemyType::Ranged => Enemy {
                id,
                entity,
                hp: ENEMY_HP,
                attack_delay: RANGED_ENEMY_ATTACK_DELAY,
                enemy_type: self,
            },
            EnemyType::MeleeSlow => Enemy {
                id,
                entity,
                hp: MELEE_SLOW_ENEMY_HEALTH,
                attack_delay: 0.0,
                enemy_type: self,
            },
        }
    }

    pub fn get_movement_speed(&self) -> f32 {
        match self {
            EnemyType::Melee => MELEE_ENEMY_MOVE_SPEED,
            EnemyType::Ranged => RANGED_ENEMY_MOVE_SPEED,
            EnemyType::MeleeSlow => MELEE_SLOW_ENEMY_MOVE_SPEED,
        }
    }

    pub fn get_attack_range(&self) -> f32 {
        match self {
            EnemyType::Melee => MELEE_ENEMY_ATTACK_RANGE,
            EnemyType::Ranged => RANGED_ENEMY_SHOOT_RANGE,
            EnemyType::MeleeSlow => MELEE_SLOW_ENEMY_ATTACK_RANGE,
        }
    }

    pub fn get_attack_damage(&self) -> f32 {
        match self {
            EnemyType::Melee => MELEE_ENEMY_DAMAGE,
            EnemyType::MeleeSlow => MELEE_SLOW_ENEMY_DAMAGE,
            EnemyType::Ranged => RANGED_ENEMY_DAMAGE,
        }
    }

    pub fn get_attack_speed(&self) -> f32 {
        match self {
            EnemyType::Melee => MELEE_ENEMY_ATTACK_DELAY,
            EnemyType::Ranged => RANGED_ENEMY_ATTACK_DELAY,
            EnemyType::MeleeSlow => MELEE_SLOW_ENEMY_ATTACK_DELAY,
        }
    }

    pub fn get_animation(&self) -> Vec<TextureId> {
        match self {
            EnemyType::Melee => Animation::Enemy.get_textures(),
            EnemyType::Ranged => Animation::RangedEnemy.get_textures(),
            EnemyType::MeleeSlow => Animation::MeleeSlowEnemy.get_textures(),
        }
    }

    pub fn get_animation_speed(&self) -> u128 {
        match self {
            EnemyType::Melee => MELEE_ENEMY_ANIMATION_SPEED,
            EnemyType::Ranged => RANGED_ENEMY_ANIMATION_SPEED,
            EnemyType::MeleeSlow => MELEE_SLOW_ENEMY_ANIMATION_SPEED,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub id: u64,
    pub entity: Entity,
    pub hp: f32,
    pub attack_delay: f32,
    pub enemy_type: EnemyType,
}
impl Sprite2D for Enemy {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, time_ellapsed: &Duration) -> f32 {
        calculate_vertical_offset(
            ENEMY_HEIGHT_CHANGE_ANIMATION_SPEED,
            ENEMY_HEIGHT_AMPLITUDE,
            ENEMY_BASE_HEIGHT_OFFSET,
            ENEMY_HEIGHT_AMPLITUDE,
            time_ellapsed,
        )
    }

    fn get_size(&self) -> f32 {
        self.entity.size
    }

    fn get_texture(&self, time_ellapsed: &Duration) -> TextureId {
        select_animation_texture(
            &self.enemy_type.get_animation(),
            self.enemy_type.get_animation_speed(),
            time_ellapsed,
        )
    }
}
impl Default for Enemy {
    fn default() -> Self {
        EnemyType::Melee.to_enemy(vec2(0.0, 0.0))
    }
}
