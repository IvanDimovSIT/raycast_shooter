use macroquad::{shapes::draw_rectangle, text::draw_text};

use crate::{
    constants::{
        BULLETS_UI_BOX_COLOR, BULLETS_UI_BOX_WIDTH_TO_HEIGHT, BULLETS_UI_POSITION, BULLETS_UI_SIZE,
        BULLETS_UI_TEXT_COLOR, MAX_BULLETS,
    },
    file_loaders::texture_manager::TextureManager,
    model::{PlayerInfo, ShootingStatus},
};

use super::Drawable;

struct BulletsDisplay {
    text: String,
}
impl Drawable for BulletsDisplay {
    fn get_z_index(&self) -> f32 {
        -1.0
    }

    fn draw(&self, screen_size: (f32, f32), _texture_manager: &TextureManager) {
        let x = BULLETS_UI_POSITION.x * screen_size.0;
        let w = BULLETS_UI_SIZE * screen_size.1 * BULLETS_UI_BOX_WIDTH_TO_HEIGHT;
        let h = BULLETS_UI_SIZE * screen_size.1;
        let y = BULLETS_UI_POSITION.y * screen_size.1;
        let font_size = BULLETS_UI_SIZE * screen_size.1 * 0.8;

        let offset_x = 0.2 * BULLETS_UI_SIZE * screen_size.0;
        let offset_y = 0.2 * BULLETS_UI_SIZE * screen_size.1;

        draw_rectangle(x - offset_x, y - h + offset_y, w, h, BULLETS_UI_BOX_COLOR);
        draw_text(&self.text, x, y, font_size, BULLETS_UI_TEXT_COLOR);
    }

    fn get_debug_info(&self) -> String {
        format!("BulletsDisplay{{text:{}}}", self.text)
    }
}

pub fn draw_bullets_display(player_info: &PlayerInfo) -> Box<dyn Drawable> {
    let text = if matches!(player_info.shooting_status, ShootingStatus::Reloading) {
        "Reloading...".to_string()
    } else {
        format!("Bullets:{}/{}", player_info.bullets, MAX_BULLETS)
    };

    Box::new(BulletsDisplay { text })
}
