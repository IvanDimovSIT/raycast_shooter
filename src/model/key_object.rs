use uuid::Uuid;

use crate::{constants::*, draw::sprite_2d::Sprite2D};

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
        ((time_ellapsed.as_millis() % KEY_ANIMATION_SPEED_MOVEMENT) as f32 * 2.0 * PI
            / KEY_ANIMATION_SPEED_MOVEMENT as f32)
            .sin()
            * 0.05
            + 0.3
    }

    fn get_size(&self) -> f32 {
        self.entity.size * KEY_DRAW_SIZE_MOD
    }

    fn get_texture(&self, time_ellapsed: &Duration) -> Texture {
        self.textures[(time_ellapsed.as_millis() / KEY_ANIMATION_SPEED_TEXTURES) as usize
            % self.textures.len()]
    }
}
