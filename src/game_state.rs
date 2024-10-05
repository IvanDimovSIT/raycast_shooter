use std::time::{Duration, Instant};

use macroquad::{
    input::{is_key_released, KeyCode},
    time::get_frame_time,
};

use crate::{
    constants::START_LEVEL,
    controller::{
        handle_events, handle_input, is_game_over, is_game_won, next_game_step, reset_state,
    },
    draw::draw_game,
    input::get_input,
    level_loader::load_level,
    model::GameObjects,
    renderer::{render_drawables, render_game_over, render_game_won, render_level_won},
    texture_manager::TextureManager,
};

pub struct GameContext {
    game_objects: GameObjects,
    texture_manager: TextureManager,
    start_time: Instant,
    level: u32,
}
impl GameContext {
    pub fn load() -> Self {
        Self {
            game_objects: load_level(START_LEVEL).expect("Can't find start level"),
            texture_manager: TextureManager::load(),
            start_time: Instant::now(),
            level: START_LEVEL,
        }
    }
}

pub enum GameState {
    Running(Box<GameContext>),
    LevelWon((Box<GameContext>, Duration)),
    GameOver,
    GameWon(Duration),
}

fn level_complete(context: Box<GameContext>) -> GameState {
    let next_level = context.level + 1;
    let duration = Instant::now().duration_since(context.start_time);
    let game_objects = load_level(next_level);
    if let Ok(level_data) = game_objects {
        GameState::LevelWon((
            Box::new(GameContext {
                game_objects: level_data,
                level: next_level,
                ..*context
            }),
            duration,
        ))
    } else {
        GameState::GameWon(duration)
    }
}

async fn normal_run(mut context: Box<GameContext>) -> (GameState, bool) {
    let delta = get_frame_time();
    let time_from_start = context.start_time.elapsed();

    context.game_objects = reset_state(context.game_objects);

    let input = get_input();

    (
        context.game_objects.player,
        context.game_objects.player_info,
    ) = handle_input(&context.game_objects, &input, delta);

    let events;
    (context.game_objects, events) = next_game_step(context.game_objects, delta);
    handle_events(&mut context.game_objects, &events);

    let to_draw = draw_game(&context.game_objects, &time_from_start);

    render_drawables(&context.texture_manager, &to_draw).await;

    let state = if is_game_over(&context.game_objects) {
        GameState::GameOver
    } else if is_game_won(&context.game_objects) {
        level_complete(context)
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

async fn level_won_run(context: Box<GameContext>, time: Duration) -> (GameState, bool) {
    render_level_won(time).await;
    if is_key_released(KeyCode::N) {
        (GameState::LevelWon((context, time)), true)
    } else if is_key_released(KeyCode::Y) {
        (
            GameState::Running(Box::new(GameContext {
                start_time: Instant::now(),
                ..*context
            })),
            false,
        )
    } else {
        (GameState::LevelWon((context, time)), false)
    }
}

pub async fn run(state: GameState) -> (GameState, bool) {
    match state {
        GameState::Running(context) => normal_run(context).await,
        GameState::GameOver => game_over_run().await,
        GameState::GameWon(time) => game_won_run(time).await,
        GameState::LevelWon((context, time)) => level_won_run(context, time).await,
    }
}
