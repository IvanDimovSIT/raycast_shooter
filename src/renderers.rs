use std::{rc::Rc, thread::sleep, time::Duration};

use macroquad::{color::{BLACK, WHITE}, input::is_key_pressed, miniquad::window::screen_size, shapes::draw_rectangle, window::{clear_background, next_frame}};

use crate::{constants::{CEILING_COLOR, DEBUG_DRAW_DELAY_MS, FLOOR_COLOR}, draw::Drawable, texture_manager::TextureManager};


fn draw_bg(screen_size: (f32, f32)) {
    clear_background(CEILING_COLOR);
    draw_rectangle(0.0, 0.5 * screen_size.1, 1.0 * screen_size.0, 1.0 * screen_size.1, FLOOR_COLOR);
}

pub async fn default_renderer(texture_manager: Rc<TextureManager>, to_draw: &[Drawable]) {
    let screen = screen_size();
    let mut draw_in_order: Vec<_> = to_draw.iter().collect();

    draw_bg(screen);
    draw_in_order.sort_by(|a,b| b.z_index.total_cmp(&a.z_index));
    for d in to_draw {
        (*d.draw)(screen, texture_manager.clone());
    }

    next_frame().await;
}

pub async fn debug_renderer(texture_manager: Rc<TextureManager>, to_draw: &[Drawable]) {
    let screen = screen_size();
    let mut draw_in_order: Vec<_> = to_draw.iter().collect();

    draw_in_order.sort_by(|a,b| b.z_index.total_cmp(&a.z_index));

    let sleep_duration = Duration::from_millis(DEBUG_DRAW_DELAY_MS);
    
    clear_background(BLACK);
    next_frame().await;
    sleep(sleep_duration);

    draw_bg(screen);
    next_frame().await;
    println!("Drawing background");
    sleep(sleep_duration);
    for i in 0..draw_in_order.len() {
        draw_bg(screen);
        draw_in_order.iter().take(i+1)
        .for_each(|d| 
            (*d.draw)(screen, texture_manager.clone())
        );
        println!("Drawing object at z:{:.4}", draw_in_order[i].z_index);
        next_frame().await;
        sleep(sleep_duration);
        if is_key_pressed(macroquad::input::KeyCode::Escape) {
            return;
        }
    }
}