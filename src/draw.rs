use std::{f32::consts::TAU, iter::once, time::Duration};

use bullets_display::draw_bullets_display;
use exit_text_sprite::create_exit_text;
use gun::draw_gun;
use health_display::draw_health_display;
use key_display::draw_key_display;
use macroquad::math::{vec2, Vec2};
use sprite_2d::{draw_sprites, Sprite2D};
use wall::draw_walls;

use crate::{
    constants::{MIN_BRIGHTNESS, VIEW_DISTANCE},
    file_loaders::texture_manager::TextureManager,
    model::{Entity, GameObjects, Player, TextureId},
};

pub mod bullets_display;
pub mod exit_text_sprite;
pub mod gun;
pub mod health_display;
pub mod key_display;
pub mod sprite_2d;
pub mod wall;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec2,
    pub look: Vec2,
}
impl Camera {
    pub fn for_player(player: &Player) -> Self {
        Self {
            position: player.entity.position,
            look: player.entity.position + player.look.normalize_or_zero() * VIEW_DISTANCE,
        }
    }
}

pub trait Drawable: Send + Sync {
    fn get_z_index(&self) -> f32;
    fn draw(&self, screen_size: (f32, f32), texture_manager: &TextureManager);
    fn get_debug_info(&self) -> String;
}

fn calculate_brightness(distance: f32) -> f32 {
    (1.0 / distance.max(0.001))
        .sqrt()
        .clamp(MIN_BRIGHTNESS, 1.0)
}

pub fn select_animation_texture(
    textures: &[TextureId],
    speed: u128,
    time_from_start: &Duration,
) -> TextureId {
    textures[(time_from_start.as_millis() / speed) as usize % textures.len()]
}

pub fn calculate_vertical_offset(
    speed: u128,
    size: f32,
    base_offset: f32,
    amplitude: f32,
    time_from_start: &Duration,
) -> f32 {
    const VERTICAL_OFFSET_COEF: f32 = 0.7;
    if speed == 0 {
        return base_offset - size * VERTICAL_OFFSET_COEF;
    }

    ((time_from_start.as_millis() % speed) as f32 * TAU / speed as f32).sin() * amplitude
        + base_offset
        - size * VERTICAL_OFFSET_COEF
}

pub fn draw_game(game_objects: &GameObjects, time_from_start: &Duration) -> Vec<Box<dyn Drawable>> {
    let camera = Camera::for_player(&game_objects.player);
    let walls_to_draw = draw_walls(&camera, &game_objects.walls);

    let exit_text_sprite = create_exit_text(game_objects);
    let sprites: Vec<&dyn Sprite2D> = game_objects
        .keys
        .iter()
        .map(|x| x as &dyn Sprite2D)
        .chain(game_objects.enemies.iter().map(|x| x as &dyn Sprite2D))
        .chain(game_objects.decorations.iter().map(|x| x as &dyn Sprite2D))
        .chain(game_objects.projectiles.iter().map(|x| x as &dyn Sprite2D))
        .chain(exit_text_sprite.iter().map(|x| x.as_ref()))
        .collect();

    let sprites_to_draw = draw_sprites(&camera, time_from_start, &sprites);

    walls_to_draw
        .into_iter()
        .chain(sprites_to_draw)
        .chain(once(draw_gun(
            time_from_start,
            game_objects.player_info.shooting_status,
        )))
        .chain(once(draw_key_display(game_objects)))
        .chain(once(draw_bullets_display(&game_objects.player_info)))
        .chain(once(draw_health_display(&game_objects.player_info)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_for_player() {
        let player = Player {
            entity: Entity {
                position: vec2(10.0, 5.0),
                size: 1.0,
            },
            look: vec2(0.0, 10.0),
        };

        let camera = Camera::for_player(&player);

        assert_eq!(camera.position, vec2(10.0, 5.0));
        assert_eq!(camera.look, vec2(10.0, 5.0 + VIEW_DISTANCE));
    }

    #[test]
    fn test_calculate_brightness() {
        let brightness1 = calculate_brightness(0.1);
        assert!((brightness1 - 1.0).abs() < f32::EPSILON);

        let brightness2 = calculate_brightness(500.0);
        assert!((brightness2 - MIN_BRIGHTNESS).abs() < f32::EPSILON);
    }

    #[test]
    fn test_select_animation_texture() {
        let textures = vec![TextureId::Debug, TextureId::Stone, TextureId::Metal];

        let speed = 100;

        let time_from_start1 = Duration::from_millis(0);
        let selected_texture1 = select_animation_texture(&textures, speed, &time_from_start1);
        assert_eq!(selected_texture1, TextureId::Debug);

        let time_from_start2 = Duration::from_millis(250);
        let selected_texture2 = select_animation_texture(&textures, speed, &time_from_start2);
        assert_eq!(selected_texture2, TextureId::Metal);

        let time_from_start3 = Duration::from_millis(50);
        let selected_texture3 = select_animation_texture(&textures, speed, &time_from_start3);
        assert_eq!(selected_texture3, TextureId::Debug);

        let time_from_start4 = Duration::from_millis(350);
        let selected_texture4 = select_animation_texture(&textures, speed, &time_from_start4);
        assert_eq!(selected_texture4, TextureId::Debug);
    }

    #[test]
    fn test_calculate_vertical_offset() {
        let speed = 0;
        let size = 2.0;
        let base_offset = 5.0;
        let amplitude = 1.0;
        let time_from_start = Duration::from_millis(500);

        let vertical_offset =
            calculate_vertical_offset(speed, size, base_offset, amplitude, &time_from_start);
        assert_eq!(vertical_offset, base_offset - size * 0.7);

        let speed2 = 1000;
        let vertical_offset2 =
            calculate_vertical_offset(speed2, size, base_offset, amplitude, &time_from_start);
        assert!(
            vertical_offset2 > base_offset - size * 0.7 - amplitude
                && vertical_offset2 < base_offset - size * 0.7 + amplitude
        );
    }
}
