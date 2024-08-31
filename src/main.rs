use std::time::Instant;

use controller::{handle_events, handle_input};
use draw::draw_game;
use input::get_input;
use macroquad::{math::vec2, miniquad::window::screen_size, time::get_frame_time};
use model::{
    enemy::Enemy, key_object::KeyObject, Entity, GameObjects, Player, PlayerInfo, Texture, Wall,
};
use renderers::render_drawables;
use service::{check_pickup_key, move_enemies_towards_player};
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

fn init_game() -> GameObjects {
    let player = Player {
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
    let keys = vec![KeyObject::new(
        Entity {
            position: vec2(0.0, 2.0),
            size: 0.8,
        },
        vec![Texture::Key1, Texture::Key2],
    )];
    let player_info = PlayerInfo::default();
    let enemies = vec![Enemy::new(
        Entity {
            position: vec2(-0.2, 3.5),
            size: 0.4,
        },
        10.0,
        vec![
            Texture::Enemy1,
            Texture::Enemy2,
            Texture::Enemy3,
            Texture::Enemy4,
            Texture::Enemy5,
            Texture::Enemy6,
            Texture::Enemy7,
            Texture::Enemy8,
        ],
    )];

    GameObjects {
        player,
        walls,
        enemies,
        keys,
        player_info,
    }
}

#[macroquad::main("Game")]
async fn main() {
    let texture_manager = TextureManager::load();
    let start_time = Instant::now();
    let mut game_objects = init_game();

    loop {
        let delta = get_frame_time();
        let time_from_start = start_time.elapsed();

        let input = get_input(screen_size());

        (game_objects.player, game_objects.player_info) =
            handle_input(&game_objects, &input, delta);

        game_objects.enemies = move_enemies_towards_player(&game_objects.player, game_objects.enemies, &game_objects.walls, delta);

        let events = check_pickup_key(&game_objects.player, &game_objects.keys);
        handle_events(&mut game_objects, &events);

        let to_draw = draw_game(&game_objects, &time_from_start);

        render_drawables(&texture_manager, &to_draw).await;
    }
}
