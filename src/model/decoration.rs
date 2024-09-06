use std::time::Duration;

use macroquad::math::Vec2;

use super::Entity;
use crate::{
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
    model::Texture,
};

#[derive(Debug, Clone)]
pub struct Decoration {
    pub entity: Entity,
    pub textures: Vec<Texture>,
    pub animation_speed: u128,
    pub life: Option<f32>,
    pub offset: f32,
}
impl Sprite2D for Decoration {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, time_ellapsed: &Duration) -> f32 {
        calculate_vertical_offset(0, self.get_size(), self.offset, 0.0, time_ellapsed)
    }

    fn get_size(&self) -> f32 {
        self.entity.size
    }

    fn get_texture(&self, time_ellapsed: &Duration) -> Texture {
        match self.textures.len() {
            0 => Texture::default(),
            1 => self.textures[0],
            _ => select_animation_texture(&self.textures, self.animation_speed, time_ellapsed)
        }
    }
}
impl Decoration {
    pub fn update(self, delta: f32) -> Option<Self> {
        if self.life.is_none() {
            return Some(self);
        }

        let new_life = self.life.unwrap() - delta;
        if new_life <= 0.0 {
            return None;
        }

        Some(Self {
            life: Some(new_life),
            ..self
        })
    }
}
