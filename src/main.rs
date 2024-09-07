use game_state::{run, GameContext, GameState};

mod constants;
mod controller;
mod draw;
mod game_state;
mod input;
mod math;
mod model;
mod renderers;
mod service;
mod texture_manager;
mod level_loader;

#[macroquad::main("Game")]
async fn main() {
    let mut game_state = GameState::Running(Box::new(GameContext::load()));

    loop {
        let should_exit;
        (game_state, should_exit) = run(game_state).await;

        if should_exit {
            break;
        }
    }
}
