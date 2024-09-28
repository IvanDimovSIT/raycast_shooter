use macroquad::color::WHITE;
use macroquad::prelude::draw_texture_ex;
use macroquad::prelude::DrawTextureParams;

use crate::{
    constants::{GUN_FIRE_ANIMATION_SPEED, GUN_POSITION, GUN_ROTATION, GUN_SIZE},
    model::{Animation, ShootingStatus},
};

use super::*;

struct GunDrawable {
    texture: Texture,
}
impl Drawable for GunDrawable {
    fn get_z_index(&self) -> f32 {
        0.0
    }

    fn draw(&self, screen_size: (f32, f32), texture_manager: &TextureManager) {
        let texture_2d = texture_manager.get_texture(self.texture);

        let gun_size = vec2(screen_size.0 * GUN_SIZE, screen_size.1 * GUN_SIZE);

        let params = DrawTextureParams {
            dest_size: Some(gun_size),
            source: None,
            rotation: GUN_ROTATION,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            texture_2d,
            (GUN_POSITION.x) * screen_size.0,
            (GUN_POSITION.y) * screen_size.1,
            WHITE,
            params,
        );
    }

    fn get_debug_info(&self) -> String {
        format!("Gun{{texture:{:?}}}", self.texture)
    }
}

pub fn draw_gun(time_ellapsed: &Duration, shooting_status: ShootingStatus) -> Box<dyn Drawable> {
    let shooting_textures = Animation::Gun.get_textures();

    let texture = if matches!(shooting_status, ShootingStatus::Shooting) {
        shooting_textures[(time_ellapsed.as_millis() / GUN_FIRE_ANIMATION_SPEED) as usize
            % shooting_textures.len()]
    } else {
        *shooting_textures
            .first()
            .expect("No shooting textures found")
    };

    Box::new(GunDrawable { texture })
}
