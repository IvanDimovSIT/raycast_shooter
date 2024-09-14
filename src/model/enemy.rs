use std::time::Duration;

use crate::model::Animation;
use macroquad::math::{vec2, Vec2};
use uuid::Uuid;

use crate::{
    constants::{ENEMY_ANIMATION_SPEED, ENEMY_DRAW_SIZE_MOD, ENEMY_HP, ENEMY_SIZE},
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
};

use super::{Entity, Texture};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub id: Uuid,
    pub entity: Entity,
    pub hp: f32,
}
impl Sprite2D for Enemy {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, time_ellapsed: &Duration) -> f32 {
        calculate_vertical_offset(300, self.get_size(), -0.1, 0.02, time_ellapsed)
    }

    fn get_size(&self) -> f32 {
        self.entity.size * ENEMY_DRAW_SIZE_MOD
    }

    fn get_texture(&self, time_ellapsed: &Duration) -> Texture {
        select_animation_texture(
            &Animation::Enemy.get_textures(),
            ENEMY_ANIMATION_SPEED,
            time_ellapsed,
        )
    }
}
impl Default for Enemy {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: ENEMY_SIZE,
            },
            hp: ENEMY_HP,
        }
    }
}
