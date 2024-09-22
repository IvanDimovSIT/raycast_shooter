use focus_manager::FocusManager;
use game_state::{run, GameContext, GameState};

mod constants;
mod controller;
mod draw;
mod focus_manager;
mod game_state;
mod input;
mod level_loader;
mod math;
mod model;
mod renderer;
mod service;
mod texture_manager;

#[macroquad::main("Game")]
async fn main() {
    let mut game_state = GameState::Running(Box::new(GameContext::load()));
    let mut focus_manager = FocusManager::new();

    loop {
        focus_manager.update();
        let should_exit;
        (game_state, should_exit) = run(game_state).await;

        if should_exit {
            break;
        }
    }
}
