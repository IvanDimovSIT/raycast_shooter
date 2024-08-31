use std::time::Duration;

use macroquad::math::Vec2;
use uuid::Uuid;

use crate::{
    constants::{ENEMY_ANIMATION_SPEED, ENEMY_DRAW_SIZE_MOD},
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
};

use super::{Entity, Texture};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub id: Uuid,
    pub entity: Entity,
    pub hp: f32,
    pub textures: Vec<Texture>,
}
impl Enemy {
    pub fn new(entity: Entity, hp: f32, textures: Vec<Texture>) -> Self {
        assert!(!textures.is_empty());
        Self {
            id: Uuid::new_v4(),
            entity,
            hp,
            textures,
        }
    }
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
        select_animation_texture(&self.textures, ENEMY_ANIMATION_SPEED, time_ellapsed)
    }
}
