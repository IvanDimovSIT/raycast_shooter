use focus_manager::FocusManager;
use game_state::{run, GameState};

mod constants;
mod controller;
mod draw;
mod file_loaders;
mod focus_manager;
mod game_state;
mod input;
mod math;
mod model;
mod renderer;
mod service;

#[macroquad::main("Game")]
async fn main() {
    let mut game_state = GameState::initialise().await;
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
