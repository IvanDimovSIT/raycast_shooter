use constants::ENTER_DEBUG_MODE_KEY;
use controller::{handle_input, GameObjects};
use draw::{draw_sprites, draw_walls, Camera, DebugSprite2D, Sprite2D};
use input::get_input;
use macroquad::{
    input::is_key_pressed, math::vec2, miniquad::window::screen_size, time::get_frame_time,
};
use model::{Entity, Player, Texture, Wall};
use renderers::{debug_renderer, default_renderer};
use texture_manager::TextureManager;

mod constants;
mod controller;
mod draw;
mod input;
mod math;
mod model;
mod renderers;
mod service;
mod texture_manager;

#[macroquad::main("Game")]
async fn main() {
    let mut player = Player {
        entity: Entity {
            position: vec2(0.0, 0.0),
            size: 0.1,
        },
        look: vec2(0.0, 1.0),
    };
    let walls = vec![
        Wall {
            texture: Texture::Stone,
            start: vec2(-1.0, -4.0),
            end: vec2(-1.0, 4.0),
        },
        Wall {
            texture: Texture::Stone,
            start: vec2(1.0, -4.0),
            end: vec2(1.0, 6.0),
        },
        Wall {
            texture: Texture::Stone,
            start: vec2(1.0, 6.0),
            end: vec2(-4.0, 6.0),
        },
    ];

    let texture_manager = TextureManager::load();
    loop {
        let delta = get_frame_time();
        let input = get_input(screen_size());

        let game_objects = GameObjects {
            player: &player,
            walls: &walls,
        };

        player = handle_input(game_objects, &input, delta);

        let camera = Camera::for_player(&player);
        let walls_to_draw = draw_walls(&camera, &walls);
        let _wall_to_draw_len = walls_to_draw.len();

        let test_object = vec![DebugSprite2D{ entity: Entity{ position: vec2(0.0, 1.0), size: 0.3 }}];
        let sprites: Vec<&dyn Sprite2D> = test_object.iter()
            .map(|x| x as &dyn Sprite2D)
            .collect();
        
        let sprites_to_draw = draw_sprites(&camera, &sprites);
        assert!((0..=1).contains(&sprites_to_draw.len()));

        let to_draw: Vec<_> = walls_to_draw.into_iter()
            .chain(sprites_to_draw)
            .collect();


        if is_key_pressed(ENTER_DEBUG_MODE_KEY) {
            debug_renderer(&texture_manager, &to_draw).await;
        } else {
            default_renderer(&texture_manager, &to_draw).await;
        };
    }
}
