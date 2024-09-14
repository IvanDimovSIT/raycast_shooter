use std::time::Duration;

use macroquad::math::Vec2;

use super::{Animation, Entity};
use crate::{
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
    model::Texture,
};

#[derive(Debug, Clone)]
pub enum DecorationGraphics {
    Animation {
        animation: Animation,
        animation_speed: u128,
    },
    Texture(Texture),
}

#[derive(Debug, Clone)]
pub struct Decoration {
    pub entity: Entity,
    pub graphics: DecorationGraphics,
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
        match self.graphics {
            DecorationGraphics::Animation {
                animation,
                animation_speed,
            } => {
                let textures = animation.get_textures();

                match textures.len() {
                    0 => Texture::default(),
                    1 => textures[0],
                    _ => select_animation_texture(&textures, animation_speed, time_ellapsed),
                }
            }
            DecorationGraphics::Texture(texture) => texture,
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
