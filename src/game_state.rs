use std::time::{Duration, Instant};

use macroquad::{
    input::{is_key_released, KeyCode},
    time::get_frame_time,
};

use crate::{
    constants::START_LEVEL,
    controller::{
        handle_events, handle_input, is_game_over, is_game_won, next_game_step, play_sounds,
        reset_state,
    },
    draw::draw_game,
    input::get_input,
    level_loader::{level_exists, load_level},
    model::{GameObjects, SoundId},
    renderer::{render_drawables, render_game_over, render_game_won, render_level_won},
    sound_manager::SoundManager,
    texture_manager::TextureManager,
};

pub struct GameContext {
    game_objects: GameObjects,
    texture_manager: TextureManager,
    sound_manager: SoundManager,
    start_time: Instant,
    level: u32,
}
impl GameContext {
    pub async fn load() -> Self {
        Self {
            game_objects: load_level(START_LEVEL).expect("Can't find start level"),
            texture_manager: TextureManager::load(),
            sound_manager: SoundManager::load().await,
            start_time: Instant::now(),
            level: START_LEVEL,
        }
    }
}

pub enum GameState {
    Running {
        context: Box<GameContext>,
    },
    LevelWon {
        context: Box<GameContext>,
        time_to_complete: Duration,
    },
    GameOver,
    GameWon {
        time_to_complete: Duration,
    },
}

fn level_complete(context: Box<GameContext>) -> GameState {
    let next_level = context.level + 1;
    let duration = Instant::now().duration_since(context.start_time);
    if level_exists(next_level) {
        let game_objects = load_level(next_level).expect("Error loading level");
        GameState::LevelWon {
            context: Box::new(GameContext {
                game_objects,
                level: next_level,
                ..*context
            }),
            time_to_complete: duration,
        }
    } else {
        GameState::GameWon {
            time_to_complete: duration,
        }
    }
}

async fn normal_run(mut context: Box<GameContext>) -> (GameState, bool) {
    let delta = get_frame_time();
    let time_from_start = context.start_time.elapsed();

    reset_state(&mut context.game_objects);

    let input = get_input();
    (
        context.game_objects.player,
        context.game_objects.player_info,
    ) = handle_input(&context.game_objects, &input, delta);

    let events;
    (context.game_objects, events) = next_game_step(context.game_objects, delta);
    handle_events(&context.sound_manager, &mut context.game_objects, &events);
    play_sounds(&mut context.sound_manager, &context.game_objects);

    let to_draw = draw_game(&context.game_objects, &time_from_start);

    render_drawables(&context.texture_manager, &to_draw).await;

    let state = if is_game_over(&context.game_objects) {
        context.sound_manager.stop_all();
        context.sound_manager.play(SoundId::Lose);
        GameState::GameOver
    } else if is_game_won(&context.game_objects) {
        context.sound_manager.stop_all();
        context.sound_manager.play(SoundId::Escape);
        level_complete(context)
    } else {
        GameState::Running { context }
    };

    (state, false)
}

async fn game_over_run() -> (GameState, bool) {
    render_game_over().await;
    if is_key_released(KeyCode::N) {
        (GameState::GameOver, true)
    } else if is_key_released(KeyCode::Y) {
        (
            GameState::Running {
                context: Box::new(GameContext::load().await),
            },
            false,
        )
    } else {
        (GameState::GameOver, false)
    }
}

async fn game_won_run(time_to_complete: Duration) -> (GameState, bool) {
    render_game_won(time_to_complete).await;
    if is_key_released(KeyCode::N) {
        (GameState::GameWon { time_to_complete }, true)
    } else if is_key_released(KeyCode::Y) {
        (
            GameState::Running {
                context: Box::new(GameContext::load().await),
            },
            false,
        )
    } else {
        (GameState::GameWon { time_to_complete }, false)
    }
}

async fn level_won_run(context: Box<GameContext>, time_to_complete: Duration) -> (GameState, bool) {
    render_level_won(time_to_complete).await;
    if is_key_released(KeyCode::N) {
        (
            GameState::LevelWon {
                context,
                time_to_complete,
            },
            true,
        )
    } else if is_key_released(KeyCode::Y) {
        (
            GameState::Running {
                context: Box::new(GameContext {
                    start_time: Instant::now(),
                    ..*context
                }),
            },
            false,
        )
    } else {
        (
            GameState::LevelWon {
                context,
                time_to_complete,
            },
            false,
        )
    }
}

pub async fn run(state: GameState) -> (GameState, bool) {
    match state {
        GameState::Running { context } => normal_run(context).await,
        GameState::GameOver => game_over_run().await,
        GameState::GameWon { time_to_complete } => game_won_run(time_to_complete).await,
        GameState::LevelWon {
            context,
            time_to_complete,
        } => level_won_run(context, time_to_complete).await,
    }
}
