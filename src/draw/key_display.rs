use macroquad::{shapes::draw_rectangle, text::draw_text};

use crate::{
    constants::{
        KEYS_UI_BOX_COLOR, KEYS_UI_BOX_WIDTH_TO_HEIGHT, KEYS_UI_FIND_EXIT_TEXT, KEYS_UI_POSITION,
        KEYS_UI_SIZE, KEYS_UI_TEXT_COLOR,
    },
    model::GameObjects,
    texture_manager::TextureManager,
};

use super::Drawable;

struct KeyDisplay {
    text: String,
}
impl Drawable for KeyDisplay {
    fn get_z_index(&self) -> f32 {
        -1.0
    }

    fn draw(&self, screen_size: (f32, f32), _texture_manager: &TextureManager) {
        let x = KEYS_UI_POSITION.x * screen_size.0;
        let w = KEYS_UI_SIZE * screen_size.1 * KEYS_UI_BOX_WIDTH_TO_HEIGHT;
        let h = KEYS_UI_SIZE * screen_size.1;
        let y = KEYS_UI_POSITION.y * screen_size.1;
        let font_size = KEYS_UI_SIZE * screen_size.1 * 0.8;

        let offset_x = 0.2 * KEYS_UI_SIZE * screen_size.0;
        let offset_y = 0.2 * KEYS_UI_SIZE * screen_size.1;

        draw_rectangle(x - offset_x, y - h + offset_y, w, h, KEYS_UI_BOX_COLOR);
        draw_text(&self.text, x, y, font_size, KEYS_UI_TEXT_COLOR);
    }

    fn get_debug_info(&self) -> String {
        format!("KeyDisplay{{text:{}}}", self.text)
    }
}

pub fn draw_key_display(game_objects: &GameObjects) -> Box<dyn Drawable> {
    let text = if game_objects.keys.is_empty() {
        KEYS_UI_FIND_EXIT_TEXT.to_string()
    } else {
        format!(
            "Keys:{}/{}",
            game_objects.player_info.picked_up_keys,
            game_objects.player_info.picked_up_keys + game_objects.keys.len()
        )
    };

    Box::new(KeyDisplay { text })
}
