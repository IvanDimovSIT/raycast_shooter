use crate::{
    constants::*,
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
};
use macroquad::math::vec2;

use super::*;

#[derive(Debug, Clone)]
pub struct KeyObject {
    pub entity: Entity,
}
impl Sprite2D for KeyObject {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, time_ellapsed: &Duration) -> f32 {
        calculate_vertical_offset(
            KEY_ANIMATION_SPEED_MOVEMENT,
            self.get_size(),
            KEY_HEIGHT_OFFSET,
            KEY_HEIGHT_ANIMATION_AMPLITUDE,
            time_ellapsed,
        )
    }

    fn get_size(&self) -> f32 {
        self.entity.size * KEY_DRAW_SIZE_MOD
    }

    fn get_texture(&self, time_ellapsed: &Duration) -> TextureId {
        select_animation_texture(
            &Animation::Key.get_textures(),
            KEY_ANIMATION_SPEED_TEXTURES,
            time_ellapsed,
        )
    }
}
impl Default for KeyObject {
    fn default() -> Self {
        Self {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: KEY_SIZE,
            },
        }
    }
}
