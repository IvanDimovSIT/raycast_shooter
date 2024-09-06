use std::time::{Duration, Instant};

use macroquad::{
    input::{is_key_released, KeyCode},
    math::vec2,
    miniquad::window::screen_size,
    time::get_frame_time,
};

use crate::{
    controller::{handle_input, is_game_over, is_game_won, next_game_step, reset_state},
    draw::draw_game,
    input::get_input,
    model::{
        enemy::Enemy, key_object::KeyObject, Entity, GameObjects, Player, PlayerInfo, Texture, Wall,
    },
    renderers::{render_drawables, render_game_over, render_game_won},
    texture_manager::TextureManager,
};

fn init_game_objects() -> GameObjects {
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
            size: 0.5,
        },
        vec![Texture::Key1, Texture::Key2],
    )];
    let player_info = PlayerInfo::default();
    let enemies = vec![Enemy::new(
        Entity {
            position: vec2(-0.2, 3.5),
            size: 0.2,
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
    let decorations = vec![];

    GameObjects {
        player,
        walls,
        enemies,
        keys,
        player_info,
        decorations,
    }
}

pub struct GameContext {
    game_objects: GameObjects,
    texture_manager: TextureManager,
    start_time: Instant,
}
impl GameContext {
    pub fn load() -> Self {
        Self {
            game_objects: init_game_objects(),
            texture_manager: TextureManager::load(),
            start_time: Instant::now(),
        }
    }
}

pub enum GameState {
    Running(Box<GameContext>),
    GameOver,
    GameWon(Duration),
}

async fn normal_run(mut context: Box<GameContext>) -> (GameState, bool) {
    let delta = get_frame_time();
    let time_from_start = context.start_time.elapsed();

    context.game_objects = reset_state(context.game_objects);

    let input = get_input(screen_size());

    (
        context.game_objects.player,
        context.game_objects.player_info,
    ) = handle_input(&context.game_objects, &input, delta);

    context.game_objects = next_game_step(context.game_objects, delta);

    let to_draw = draw_game(&context.game_objects, &time_from_start);

    render_drawables(&context.texture_manager, &to_draw).await;

    let state = if is_game_over(&context.game_objects) {
        GameState::GameOver
    } else if is_game_won(&context.game_objects) {
        GameState::GameWon(Instant::now().duration_since(context.start_time))
    } else {
        GameState::Running(context)
    };

    (state, false)
}

async fn game_over_run() -> (GameState, bool) {
    render_game_over().await;
    if is_key_released(KeyCode::N) {
        (GameState::GameOver, true)
    } else if is_key_released(KeyCode::Y) {
        (GameState::Running(Box::new(GameContext::load())), false)
    } else {
        (GameState::GameOver, false)
    }
}

async fn game_won_run(time: Duration) -> (GameState, bool) {
    render_game_won(time).await;
    if is_key_released(KeyCode::N) {
        (GameState::GameWon(time), true)
    } else if is_key_released(KeyCode::Y) {
        (GameState::Running(Box::new(GameContext::load())), false)
    } else {
        (GameState::GameWon(time), false)
    }
}

pub async fn run(state: GameState) -> (GameState, bool) {
    match state {
        GameState::Running(context) => normal_run(context).await,
        GameState::GameOver => game_over_run().await,
        GameState::GameWon(time) => game_won_run(time).await,
    }
}
