use crate::{
    constants::{
        CEILING_COLOR, DEBUG_DRAW_DELAY_MS, DEBUG_INITAL_DRAW_DELAY_MS, ENTER_DEBUG_MODE_KEY,
        EXIT_DEBUG_MODE_KEY, FLOOR_COLOR, GAME_OVER_TEXT, GAME_OVER_TEXT_SIZE, GAME_WON_TEXT,
        GAME_WON_TEXT_SIZE, GAME_WON_TIME_TEXT_SIZE, GAME_WON_TIME_TEXT_X_OFFSET,
        LEVEL_WON_NEXT_LEVEL_TEXT, LEVEL_WON_NEXT_LEVEL_TEXT_SIZE, LEVEL_WON_TEXT,
        LEVEL_WON_TEXT_SIZE, LEVEL_WON_TIME_TEXT_SIZE, LEVEL_WON_TIME_TEXT_X_OFFSET,
        TRY_AGAIN_TEXT, TRY_AGAIN_TEXT_SIZE, TRY_AGAIN_WON_TEXT, TRY_AGAIN_WON_TEXT_SIZE,
    },
    draw::Drawable,
    file_loaders::texture_manager::TextureManager,
};
use macroquad::{
    color::{BLACK, DARKBLUE, ORANGE, RED, WHITE},
    input::{is_key_pressed, is_key_released},
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    text::draw_text,
    window::{clear_background, next_frame},
};
use std::{thread::sleep, time::Duration};

fn draw_bg(screen_size: (f32, f32)) {
    clear_background(CEILING_COLOR);
    draw_rectangle(
        0.0,
        0.5 * screen_size.1,
        1.0 * screen_size.0,
        1.0 * screen_size.1,
        FLOOR_COLOR,
    );
}

fn sort_drawables(to_draw: &[Box<dyn Drawable>]) -> Vec<&Box<dyn Drawable>> {
    let mut draw_in_order: Vec<_> = to_draw.iter().collect();
    draw_in_order.sort_by(|a, b| b.get_z_index().total_cmp(&a.get_z_index()));
    draw_in_order
}

async fn default_renderer(
    texture_manager: &TextureManager,
    screen: (f32, f32),
    draw_in_order: &[&Box<dyn Drawable>],
) {
    draw_bg(screen);
    for d in draw_in_order {
        d.draw(screen, texture_manager);
    }
    next_frame().await;
}

async fn debug_renderer(
    texture_manager: &TextureManager,
    screen: (f32, f32),
    draw_in_order: &[&Box<dyn Drawable>],
) {
    let initial_delay = Duration::from_millis(DEBUG_INITAL_DRAW_DELAY_MS);
    let sleep_duration = Duration::from_millis(DEBUG_DRAW_DELAY_MS);
    clear_background(BLACK);
    next_frame().await;
    sleep(initial_delay);
    draw_bg(screen);
    next_frame().await;
    println!("Drawing background");
    sleep(initial_delay);
    for i in 0..draw_in_order.len() {
        draw_bg(screen);
        draw_in_order
            .iter()
            .take(i + 1)
            .for_each(|d| d.draw(screen, texture_manager));
        println!(
            "Drawing {} at z:{:.4}",
            draw_in_order[i].get_debug_info(),
            draw_in_order[i].get_z_index()
        );
        next_frame().await;
        sleep(sleep_duration);
        if is_key_pressed(EXIT_DEBUG_MODE_KEY) {
            return;
        }
    }
    println!("Drew {} images", draw_in_order.len());
}

pub async fn render_drawables(texture_manager: &TextureManager, to_draw: &[Box<dyn Drawable>]) {
    let draw_in_order = sort_drawables(to_draw);
    let screen = screen_size();

    if is_key_released(ENTER_DEBUG_MODE_KEY) {
        debug_renderer(texture_manager, screen, &draw_in_order).await;
    } else {
        default_renderer(texture_manager, screen, &draw_in_order).await;
    };
}

pub async fn render_game_over() {
    let screen = screen_size();
    clear_background(RED);

    let x1 = (0.5 - GAME_OVER_TEXT.len() as f32 * GAME_OVER_TEXT_SIZE * 0.25) * screen.0;
    let x2 = (0.5 - TRY_AGAIN_TEXT.len() as f32 * TRY_AGAIN_TEXT_SIZE * 0.25) * screen.0;

    draw_text(
        GAME_OVER_TEXT,
        x1,
        0.5 * screen.1,
        GAME_OVER_TEXT_SIZE * screen.0,
        WHITE,
    );

    draw_text(
        TRY_AGAIN_TEXT,
        x2,
        0.8 * screen.1,
        TRY_AGAIN_TEXT_SIZE * screen.0,
        WHITE,
    );

    next_frame().await;
}

fn time_to_text(time: Duration) -> String {
    let minutes = time.as_secs() / 60;
    let secs = time.as_secs() % 60;
    format!("{:02}:{:02}", minutes, secs)
}

pub async fn render_game_won(time: Duration, best_time: Duration) {
    let screen = screen_size();
    clear_background(ORANGE);

    let time_text = format!(
        "Time: {} (Best:{})",
        time_to_text(time),
        time_to_text(best_time)
    );

    let x1 = (0.5 - GAME_WON_TEXT.len() as f32 * GAME_WON_TEXT_SIZE * 0.25) * screen.0;
    let x2 = (GAME_WON_TIME_TEXT_X_OFFSET
        - time_text.len() as f32 * GAME_WON_TIME_TEXT_SIZE * 0.25)
        * screen.0;
    let x3 = (0.5 - TRY_AGAIN_WON_TEXT.len() as f32 * TRY_AGAIN_WON_TEXT_SIZE * 0.25) * screen.0;

    draw_text(
        GAME_WON_TEXT,
        x1,
        0.4 * screen.1,
        GAME_WON_TEXT_SIZE * screen.0,
        WHITE,
    );
    draw_text(
        &time_text,
        x2,
        0.6 * screen.1,
        GAME_WON_TIME_TEXT_SIZE * screen.0,
        WHITE,
    );
    draw_text(
        TRY_AGAIN_WON_TEXT,
        x3,
        0.85 * screen.1,
        TRY_AGAIN_WON_TEXT_SIZE * screen.0,
        WHITE,
    );

    next_frame().await;
}

pub async fn render_level_won(time: Duration, best_time: Duration) {
    let screen = screen_size();
    clear_background(DARKBLUE);

    let time_text = format!(
        "Time: {} (Best:{})",
        time_to_text(time),
        time_to_text(best_time)
    );

    let x1 = (0.5 - LEVEL_WON_TEXT.len() as f32 * LEVEL_WON_TEXT_SIZE * 0.25) * screen.0;
    let x2 = (LEVEL_WON_TIME_TEXT_X_OFFSET
        - time_text.len() as f32 * LEVEL_WON_TIME_TEXT_SIZE * 0.25)
        * screen.0;
    let x3 = (0.5 - LEVEL_WON_NEXT_LEVEL_TEXT.len() as f32 * LEVEL_WON_NEXT_LEVEL_TEXT_SIZE * 0.25)
        * screen.0;

    draw_text(
        LEVEL_WON_TEXT,
        x1,
        0.4 * screen.1,
        LEVEL_WON_TEXT_SIZE * screen.0,
        WHITE,
    );
    draw_text(
        &time_text,
        x2,
        0.6 * screen.1,
        LEVEL_WON_TIME_TEXT_SIZE * screen.0,
        WHITE,
    );
    draw_text(
        LEVEL_WON_NEXT_LEVEL_TEXT,
        x3,
        0.85 * screen.1,
        LEVEL_WON_NEXT_LEVEL_TEXT_SIZE * screen.0,
        WHITE,
    );

    next_frame().await;
}
