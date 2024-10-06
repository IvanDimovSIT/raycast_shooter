use std::time::Duration;

use macroquad::math::Vec2;

use crate::{
    constants::{EXIT_TEXT_HEIGHT_OFFSET, EXIT_TEXT_SIZE},
    math::check_circles_collide,
    model::{Entity, GameObjects, TextureId},
};

use super::sprite_2d::Sprite2D;

struct ExitTextSprite {
    entity: Entity,
}
impl Sprite2D for ExitTextSprite {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, _time_ellapsed: &Duration) -> f32 {
        EXIT_TEXT_HEIGHT_OFFSET
    }

    fn get_size(&self) -> f32 {
        self.entity.size
    }

    fn get_texture(&self, _time_ellapsed: &Duration) -> TextureId {
        TextureId::TextFindTheKeys
    }
}

pub fn create_exit_text(game_objects: &GameObjects) -> Option<Box<dyn Sprite2D>> {
    let nearest = game_objects
        .exit_triggers
        .iter()
        .filter(|exit| {
            !game_objects.keys.is_empty()
                && check_circles_collide(
                    game_objects.player.entity.position,
                    game_objects.player.entity.size,
                    exit.position,
                    exit.size,
                )
        })
        .min_by(|a, b| {
            a.position
                .distance(game_objects.player.entity.position)
                .total_cmp(&b.position.distance(game_objects.player.entity.position))
        })?;

    Some(Box::new(ExitTextSprite {
        entity: Entity {
            position: nearest.position,
            size: EXIT_TEXT_SIZE,
        },
    }))
}
