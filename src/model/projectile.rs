use std::time::Duration;

use macroquad::math::Vec2;

use crate::{constants::PROJECTILE_OFFSET, draw::sprite_2d::Sprite2D};

use super::{Entity, TextureId};

#[derive(Debug, Clone)]
pub struct Projectile {
    pub entity: Entity,
    pub direction: Vec2,
    pub damage: f32,
    pub texture: TextureId,
}
impl Sprite2D for Projectile {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, _time_ellapsed: &Duration) -> f32 {
        PROJECTILE_OFFSET
    }

    fn get_size(&self) -> f32 {
        self.entity.size
    }

    fn get_texture(&self, _time_ellapsed: &Duration) -> TextureId {
        self.texture
    }
}
