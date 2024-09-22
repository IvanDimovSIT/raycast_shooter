use crate::constants::FOV;
use macroquad::prelude::draw_texture_ex;
use macroquad::prelude::Color;
use macroquad::prelude::DrawTextureParams;
use std::time::Duration;

use super::*;
use rayon::prelude::*;

struct Sprite2dDrawable {
    texture: Texture,
    x: f32,
    z_index: f32,
    brightness: f32,
    vertical_offset: f32,
    size: f32,
}
impl Drawable for Sprite2dDrawable {
    fn get_z_index(&self) -> f32 {
        self.z_index
    }

    fn draw(&self, screen_size: (f32, f32), texture_manager: &TextureManager) {
        let texture = texture_manager.get_texture(self.texture);
        let color = Color {
            r: self.brightness,
            g: self.brightness,
            b: self.brightness,
            a: 1.0,
        };
        let texture_size = texture.size();
        let texture_scale = self.size / texture_size.x;
        let dest_size = Some(vec2(
            texture_size.x * texture_scale * screen_size.0,
            texture_size.y * texture_scale * screen_size.1,
        ));

        let params = DrawTextureParams {
            dest_size,
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            texture,
            self.x * screen_size.0,
            (0.5 + self.vertical_offset) * screen_size.1,
            color,
            params,
        );
    }

    fn get_debug_info(&self) -> String {
        format!(
            "Sprite2D{{x:{:.4} size:{:.4} offset:{:.4} brightness:{:.4}}}",
            self.x, self.size, self.vertical_offset, self.brightness
        )
    }
}

pub trait Sprite2D: Send + Sync {
    fn get_position(&self) -> Vec2;
    fn get_vertical_offset(&self, time_ellapsed: &Duration) -> f32;
    fn get_size(&self) -> f32;
    fn get_texture(&self, time_ellapsed: &Duration) -> Texture;
}

pub struct DebugSprite2D {
    pub entity: Entity,
}
impl Sprite2D for DebugSprite2D {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, _time_ellapsed: &Duration) -> f32 {
        0.0
    }

    fn get_size(&self) -> f32 {
        0.1
    }

    fn get_texture(&self, _time_ellapsed: &Duration) -> Texture {
        Texture::Debug
    }
}

fn sprite_to_drawable(
    time_ellapsed: &Duration,
    camera_pos: Vec2,
    camera_look: Vec2,
    sprite: &dyn Sprite2D,
) -> Option<Box<dyn Drawable>> {
    let v = sprite.get_position() - camera_pos;
    let distance = v.length();
    let sprite_size = sprite.get_size() / distance;
    let half_sprite_size = sprite_size / 2.0;

    let angle = camera_look.angle_between(v);

    if (angle.abs() - half_sprite_size) > FOV / 2.0 {
        return None;
    }

    let tan_half_fov = (FOV / 2.0).tan();
    let screen_x = 0.5 - (angle.tan() / tan_half_fov) * 0.5 - half_sprite_size;

    Some(Box::new(Sprite2dDrawable {
        texture: sprite.get_texture(time_ellapsed),
        x: screen_x,
        z_index: distance,
        brightness: calculate_brightness(distance),
        vertical_offset: sprite.get_vertical_offset(time_ellapsed) * (1.0 / distance),
        size: sprite_size,
    }))
}

pub fn draw_sprites(
    camera: &Camera,
    time_ellapsed: &Duration,
    sprites: &[&dyn Sprite2D],
) -> Vec<Box<dyn Drawable>> {
    let camera_pos = camera.position;
    let camera_look = camera.look.normalize_or_zero();

    sprites
        .par_iter()
        .filter_map(|sprite| sprite_to_drawable(time_ellapsed, camera_pos, camera_look, *sprite))
        .collect()
}
