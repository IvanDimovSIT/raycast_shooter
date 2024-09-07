use macroquad::math::vec2;
use uuid::Uuid;

use crate::{
    constants::*,
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
};

use super::*;

#[derive(Debug)]
pub struct KeyObject {
    pub id: Uuid,
    pub textures: Vec<Texture>,
    pub entity: Entity,
}
impl KeyObject {
    pub fn new(entity: Entity, textures: Vec<Texture>) -> Self {
        Self {
            id: Uuid::new_v4(),
            textures,
            entity,
        }
    }
}
impl Sprite2D for KeyObject {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, time_ellapsed: &Duration) -> f32 {
        calculate_vertical_offset(
            KEY_ANIMATION_SPEED_MOVEMENT,
            self.get_size(),
            0.3,
            0.05,
            time_ellapsed,
        )
    }

    fn get_size(&self) -> f32 {
        self.entity.size * KEY_DRAW_SIZE_MOD
    }

    fn get_texture(&self, time_ellapsed: &Duration) -> Texture {
        select_animation_texture(&self.textures, KEY_ANIMATION_SPEED_TEXTURES, time_ellapsed)
    }
}
impl Default for KeyObject {
    fn default() -> Self {
        Self { 
            id: Uuid::new_v4(),
            textures: vec![Texture::Key1, Texture::Key2],
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: KEY_SIZE
            } 
        }
    }
}