use macroquad::shapes::draw_rectangle;

use crate::{
    constants::{
        HEALTH_DISPLAY_BACKGROUND_COLOR, HEALTH_DISPLAY_BORDER_SIZE, HEALTH_DISPLAY_COLOR,
        HEALTH_DISPLAY_HEIGHT, HEALTH_DISPLAY_POSITION, HEALTH_DISPLAY_WIDTH, PLAYER_MAX_HEALTH,
    },
    file_loaders::texture_manager::TextureManager,
    model::PlayerInfo,
};

use super::Drawable;

struct HealthDisplay {
    bar_length: f32,
}
impl Drawable for HealthDisplay {
    fn get_z_index(&self) -> f32 {
        -1.0
    }

    fn draw(&self, screen_size: (f32, f32), _texture_manager: &TextureManager) {
        let bg_x = (HEALTH_DISPLAY_POSITION.x - HEALTH_DISPLAY_BORDER_SIZE) * screen_size.0;
        let bg_y = (HEALTH_DISPLAY_POSITION.y - HEALTH_DISPLAY_BORDER_SIZE) * screen_size.1;
        let bg_width = (HEALTH_DISPLAY_BORDER_SIZE * 2.0 + HEALTH_DISPLAY_WIDTH) * screen_size.0;
        let bg_height = (HEALTH_DISPLAY_BORDER_SIZE * 2.0 + HEALTH_DISPLAY_HEIGHT) * screen_size.1;

        draw_rectangle(
            bg_x,
            bg_y,
            bg_width,
            bg_height,
            HEALTH_DISPLAY_BACKGROUND_COLOR,
        );
        let x = HEALTH_DISPLAY_POSITION.x * screen_size.0;
        let y = HEALTH_DISPLAY_POSITION.y * screen_size.1;
        let width = self.bar_length * screen_size.0;
        let height = HEALTH_DISPLAY_HEIGHT * screen_size.1;

        draw_rectangle(x, y, width, height, HEALTH_DISPLAY_COLOR);
    }

    fn get_debug_info(&self) -> String {
        format!("HealthDisplay{{bar_length:{}}}", self.bar_length)
    }
}

pub fn draw_health_display(player_info: &PlayerInfo) -> Box<dyn Drawable> {
    Box::new(HealthDisplay {
        bar_length: (player_info.health / PLAYER_MAX_HEALTH) * HEALTH_DISPLAY_WIDTH,
    })
}
