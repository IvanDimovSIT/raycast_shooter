use std::{f32::consts::TAU, iter::once, time::Duration};

use gun::draw_gun;
use macroquad::{
    color::Color,
    math::{vec2, Rect, Vec2},
    texture::{self, draw_texture_ex, DrawTextureParams},
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use sprite_2d::{draw_sprites, Sprite2D};
use wall::draw_walls;

use crate::{
    constants::{FOV, HORIZONTAL_WALL_SEGEMENTS, MIN_BRIGHTNESS, VIEW_DISTANCE, WALL_RESOLUTION},
    math::{find_intersection, find_perpendicular_vector},
    model::{Entity, GameObjects, Player, Texture, Wall},
    texture_manager::TextureManager,
};

pub mod gun;
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
            look: player.entity.position + player.look * VIEW_DISTANCE,
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
    textures: &[Texture],
    speed: u128,
    time_from_start: &Duration,
) -> Texture {
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
        + base_offset - size * VERTICAL_OFFSET_COEF
}

pub fn draw_game(game_objects: &GameObjects, time_from_start: &Duration) -> Vec<Box<dyn Drawable>> {
    let camera = Camera::for_player(&game_objects.player);
    let walls_to_draw = draw_walls(&camera, &game_objects.walls);

    let sprites: Vec<&dyn Sprite2D> = game_objects
        .keys
        .iter()
        .map(|x| x as &dyn Sprite2D)
        .chain(game_objects.enemies.iter().map(|x| x as &dyn Sprite2D))
        .collect();

    let sprites_to_draw = draw_sprites(&camera, time_from_start, &sprites);

    walls_to_draw
        .into_iter()
        .chain(sprites_to_draw)
        .chain(once(draw_gun(
            time_from_start,
            game_objects.player_info.is_shooting,
        )))
        .collect()
}
