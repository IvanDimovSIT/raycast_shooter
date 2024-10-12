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
    file_loaders::{
        level_loader::{level_exists, load_level},
        sound_manager::SoundManager,
        texture_manager::TextureManager,
    },
    input::get_input,
    model::{GameObjects, SoundId},
    renderer::{render_drawables, render_game_over, render_game_won, render_level_won},
};

pub struct ResourceManager {
    texture_manager: TextureManager,
    sound_manager: SoundManager,
}
impl ResourceManager {
    async fn load() -> Self {
        Self {
            texture_manager: TextureManager::load(),
            sound_manager: SoundManager::load().await,
        }
    }
}

pub struct GameContext {
    game_objects: GameObjects,
    start_time: Instant,
    level: u32,
}
impl GameContext {
    fn load() -> Self {
        Self {
            game_objects: load_level(START_LEVEL).expect("Can't find start level"),
            start_time: Instant::now(),
            level: START_LEVEL,
        }
    }
}

pub enum GameState {
    Running {
        context: Box<GameContext>,
        resource_manager: ResourceManager,
    },
    LevelWon {
        context: Box<GameContext>,
        time_to_complete: Duration,
        resource_manager: ResourceManager,
    },
    GameOver {
        context: Box<GameContext>,
        resource_manager: ResourceManager,
    },
    GameWon {
        time_to_complete: Duration,
        resource_manager: ResourceManager,
    },
}
impl GameState {
    pub async fn initialise() -> Self {
        Self::Running {
            context: Box::new(GameContext::load()),
            resource_manager: ResourceManager::load().await,
        }
    }
}

fn level_complete(context: Box<GameContext>, resource_manager: ResourceManager) -> GameState {
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
            resource_manager,
        }
    } else {
        GameState::GameWon {
            time_to_complete: duration,
            resource_manager,
        }
    }
}

async fn normal_run(
    mut context: Box<GameContext>,
    mut resource_manager: ResourceManager,
) -> (GameState, bool) {
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
    handle_events(
        &resource_manager.sound_manager,
        &mut context.game_objects,
        &events,
    );
    play_sounds(&mut resource_manager.sound_manager, &context.game_objects);

    let to_draw = draw_game(&context.game_objects, &time_from_start);

    render_drawables(&resource_manager.texture_manager, &to_draw).await;

    let state = if is_game_over(&context.game_objects) {
        resource_manager.sound_manager.stop_all();
        resource_manager.sound_manager.play(SoundId::Lose);
        GameState::GameOver {
            context,
            resource_manager,
        }
    } else if is_game_won(&context.game_objects) {
        resource_manager.sound_manager.stop_all();
        resource_manager.sound_manager.play(SoundId::Escape);
        level_complete(context, resource_manager)
    } else {
        GameState::Running {
            context,
            resource_manager,
        }
    };

    (state, false)
}

async fn game_over_run(
    context: Box<GameContext>,
    resource_manager: ResourceManager,
) -> (GameState, bool) {
    render_game_over().await;
    if is_key_released(KeyCode::N) {
        (
            GameState::GameOver {
                context,
                resource_manager,
            },
            true,
        )
    } else if is_key_released(KeyCode::Y) {
        let game_objects = load_level(context.level);
        if game_objects.is_err() {
            println!("Error reloading level {}", context.level);
            return (
                GameState::GameOver {
                    context,
                    resource_manager,
                },
                false,
            );
        }

        (
            GameState::Running {
                context: Box::new(GameContext {
                    game_objects: game_objects.unwrap(),
                    start_time: Instant::now(),
                    level: context.level,
                }),
                resource_manager,
            },
            false,
        )
    } else {
        (
            GameState::GameOver {
                context,
                resource_manager,
            },
            false,
        )
    }
}

async fn game_won_run(
    time_to_complete: Duration,
    resource_manager: ResourceManager,
) -> (GameState, bool) {
    render_game_won(time_to_complete).await;
    if is_key_released(KeyCode::N) {
        (
            GameState::GameWon {
                time_to_complete,
                resource_manager,
            },
            true,
        )
    } else if is_key_released(KeyCode::Y) {
        (
            GameState::Running {
                context: Box::new(GameContext::load()),
                resource_manager,
            },
            false,
        )
    } else {
        (
            GameState::GameWon {
                time_to_complete,
                resource_manager,
            },
            false,
        )
    }
}

async fn level_won_run(
    context: Box<GameContext>,
    time_to_complete: Duration,
    resource_manager: ResourceManager,
) -> (GameState, bool) {
    render_level_won(time_to_complete).await;
    if is_key_released(KeyCode::N) {
        (
            GameState::LevelWon {
                context,
                time_to_complete,
                resource_manager,
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
                resource_manager,
            },
            false,
        )
    } else {
        (
            GameState::LevelWon {
                context,
                time_to_complete,
                resource_manager,
            },
            false,
        )
    }
}

pub async fn run(state: GameState) -> (GameState, bool) {
    match state {
        GameState::Running {
            context,
            resource_manager,
        } => normal_run(context, resource_manager).await,
        GameState::GameOver {
            context,
            resource_manager,
        } => game_over_run(context, resource_manager).await,
        GameState::GameWon {
            time_to_complete,
            resource_manager,
        } => game_won_run(time_to_complete, resource_manager).await,
        GameState::LevelWon {
            context,
            time_to_complete,
            resource_manager,
        } => level_won_run(context, time_to_complete, resource_manager).await,
    }
}
