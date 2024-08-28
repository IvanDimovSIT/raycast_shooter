use std::future::Future;

use constants::VIEW_DISTANCE;
use controller::{handle_input, GameObjects};
use draw::{draw_walls, Camera, Drawable};
use input::get_input;
use macroquad::{camera, color::WHITE, input::{is_key_down, is_key_pressed}, math::vec2, miniquad::window::screen_size, time::get_frame_time, window::{clear_background, next_frame}};
use model::{Entity, Player, Texture, Wall};
use renderers::{debug_renderer, default_renderer};
use texture_manager::TextureManager;

mod input;
mod model;
mod draw;
mod constants;
mod controller;
mod math;
mod service;
mod renderers;
mod texture_manager;

#[macroquad::main("Game")]
async fn main() {
    let mut player = Player{
        entity: Entity{ position: vec2(0.0, 0.0), size: 0.1 },
        look: vec2(0.0, 1.0)
    };
    let walls = vec![
        Wall{ texture: Texture::Stone, start: vec2(0.6, 8.0), end: vec2(0.6, 14.0) },
        Wall{ texture: Texture::Stone, start: vec2(-0.5, 2.0), end: vec2(0.6, 8.0) },
        Wall{ texture: Texture::Stone, start: vec2(-0.5, 2.0), end: vec2(-0.5, -4.0) },
    ];

    let texture_manager = TextureManager::load();
    loop {
        let delta = get_frame_time();
        let input = get_input(screen_size());

        let game_objects = GameObjects { player: &player, walls: &walls };

        player = handle_input(game_objects, &input, delta);
        

        let camera = Camera::for_player(&player);
        let to_draw = draw_walls(&camera, &walls);
    
        if is_key_pressed(macroquad::input::KeyCode::GraveAccent) {
            debug_renderer(texture_manager.clone(), &to_draw).await;
        }else{
            default_renderer(texture_manager.clone(), &to_draw).await;
        };
    }
}
