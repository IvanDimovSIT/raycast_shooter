use std::{f32::consts::PI, fmt::{format, Debug}};

use macroquad::{
    color::Color,
    math::{vec2, Rect, Vec2},
    texture::{self, draw_texture, draw_texture_ex, DrawTextureParams},
};
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    constants::{FOV, HORIZONTAL_WALL_SEGEMENTS, MIN_BRIGHTNESS, VIEW_DISTANCE},
    math::find_intersection,
    model::{Entity, Player, Texture, Wall},
    texture_manager::TextureManager,
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec2,
    pub look: Vec2,
}
impl Camera {
    pub fn for_player(player: &Player) -> Self {
        Self {
            position: player.entity.position,
            look: player.entity.position + player.look * VIEW_DISTANCE,
        }
    }
}

pub trait Drawable: Send + Sync {
    fn get_z_index(&self) -> f32;
    fn draw(&self, screen_size: (f32, f32), texture_manager: &TextureManager);
    fn get_debug_info(&self) -> String;
}

struct Sprite2dDrawable{
    texture: Texture,
    x: f32,
    z_index: f32,
    brightness: f32,
    vertical_offset: f32,
    size: f32
}
impl Drawable for Sprite2dDrawable {
    fn get_z_index(&self) -> f32 {
        self.z_index
    }

    fn draw(&self, screen_size: (f32, f32), texture_manager: &TextureManager) {
        let texture = texture_manager.get_texture(self.texture);
        let color = Color { r: self.brightness, g: self.brightness, b: self.brightness, a: 1.0 };
        let texture_size = texture.size();
        let texture_scale = self.size/texture_size.x;
        let dest_size = Some(vec2(
            texture_size.x*texture_scale*screen_size.0,
            texture_size.y*texture_scale*screen_size.1
        ));

        let params = DrawTextureParams{
            dest_size,
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            texture,
            self.x*screen_size.0,
            (0.5 + self.vertical_offset)*screen_size.1,
            color,
            params
        );
    }
    
    fn get_debug_info(&self) -> String {
        format!("Sprite2dDrawable{{x:{:.4} size:{:.4} brightness:{:.4}}}", self.x, self.size, self.brightness)
    }
}

pub trait Sprite2D {
    fn get_position(&self) -> Vec2;
    fn get_vertical_offset(&self) -> f32;
    fn get_size(&self) -> f32;
    fn get_texture(&self) -> Texture;
}

pub struct DebugSprite2D {
    pub entity: Entity
}
impl Sprite2D for DebugSprite2D {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self) -> f32 {
        0.0
    }

    fn get_size(&self) -> f32 {
        0.1
    }

    fn get_texture(&self) -> Texture {
        Texture::Debug
    }
}

struct WallDrawable {
    distance: f32,
    brightness: f32,
    x: usize,
    texture: Texture,
    relative_position: f32,
}
impl Drawable for WallDrawable {
    fn get_z_index(&self) -> f32 {
        self.distance
    }

    fn draw(&self, screen_size: (f32, f32), texture_manager: &TextureManager) {
        draw_wall(
            screen_size,
            texture_manager,
            self.x,
            self.distance,
            self.brightness,
            self.texture,
            self.relative_position,
        );
    }
    
    fn get_debug_info(&self) -> String {
        format!("Wall{{x:{:.4} brightnes::{:.4} relative_position:{:.4}}}", self.x, self.brightness, self.relative_position)
    }
}

#[derive(Debug)]
struct RayHit {
    distance_to_ray: f32,
    texture: Texture,
    relative_position: f32, // 0.0 start of wall, 1.0 end of wall
}

fn create_rays(look: Vec2) -> Vec<Vec2> {
    let half_fov = FOV / 2.0;
    let segment_angle = FOV / HORIZONTAL_WALL_SEGEMENTS as f32;

    (0..=HORIZONTAL_WALL_SEGEMENTS)
        .rev()
        .map(|x| {
            let angle = -half_fov + x as f32 * segment_angle;
            Vec2::new(
                look.x * angle.cos() - look.y * angle.sin(),
                look.x * angle.sin() + look.y * angle.cos(),
            )
        })
        .collect()
}

fn calculate_relative_position(hit: Vec2, wall_start: Vec2, wall_end: Vec2) -> f32 {
    let wall_length = wall_start.distance(wall_end);
    let hit_distance = wall_start.distance(hit);
    hit_distance / wall_length
}

fn cast_ray(ray_origin: Vec2, ray_direction: Vec2, walls: &[Wall]) -> Option<RayHit> {
    walls
        .iter()
        .filter_map(|wall| {
            Some((
                wall,
                find_intersection(ray_origin, ray_direction, wall.start, wall.end)?,
            ))
        })
        .map(|(wall, point)| RayHit {
            distance_to_ray: point.distance(ray_origin),
            texture: wall.texture,
            relative_position: calculate_relative_position(point, wall.start, wall.end),
        })
        .min_by(|a, b| a.distance_to_ray.total_cmp(&b.distance_to_ray))
}

fn calculate_brightness(distance: f32) -> f32 {
    (1.0 / distance.max(0.001))
        .sqrt()
        .clamp(MIN_BRIGHTNESS, 1.0)
}

fn draw_wall(
    screen_size: (f32, f32),
    texture_manager: &TextureManager,
    x: usize,
    distance: f32,
    brightness: f32,
    texture: Texture,
    relative_position: f32,
) {
    let height = 1.0 / distance;
    let width = 1.0 / HORIZONTAL_WALL_SEGEMENTS as f32;
    let center_x = x as f32 / HORIZONTAL_WALL_SEGEMENTS as f32 + width / 2.0;
    let center_y = 0.5;

    let texture_2d = texture_manager.get_texture(texture);
    let x = center_x - width / 2.0;
    let y = center_y - height / 2.0;

    let source = Rect::new(
        texture_2d.width() * relative_position,
        0.0,
        texture_2d.width() / HORIZONTAL_WALL_SEGEMENTS as f32,
        texture_2d.height(),
    );

    let params = texture::DrawTextureParams {
        dest_size: Some(vec2(width * screen_size.0, height * screen_size.1)),
        source: Some(source),
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };
    draw_texture_ex(
        texture_2d,
        x * screen_size.0,
        y * screen_size.1,
        Color::new(brightness, brightness, brightness, 1.0),
        params,
    );
}

pub fn draw_walls(camera: &Camera, walls: &[Wall]) -> Vec<Box<dyn Drawable>> {
    create_rays(camera.look)
        .iter()
        .enumerate()
        .par_bridge()
        .filter_map(|(x, ray)| -> Option<Box<dyn Drawable>> {
            let hit = cast_ray(camera.position, *ray, walls)?;

            let distance = hit.distance_to_ray;
            let brightness = calculate_brightness(distance);
            let texture = hit.texture;
            let relative_position = hit.relative_position;

            let drawable = WallDrawable {
                distance,
                brightness,
                x,
                texture,
                relative_position,
            };

            Some(Box::new(drawable))
        })
        .collect()
}


fn sprite_to_drawable(camera_pos: Vec2, camera_look: Vec2, sprite: &dyn Sprite2D) -> Option<Box<dyn Drawable>> {
    let v = sprite.get_position() - camera_pos;
    let v_norm = v.normalize_or_zero();
    let dot_p = v_norm.dot(camera_look);

    let sprite_half_width = sprite.get_size()/2.0;
    if dot_p < (FOV/2.0).cos() - sprite_half_width {
        return None;
    }

    let distance = camera_pos.distance(sprite.get_position());
    let camera_look_perp = vec2(-camera_look.y, camera_look.x);
    let cos_fov = (FOV/2.0).cos();

    let screen_x = 0.5 - (v.dot(camera_look_perp) / distance) / (dot_p * cos_fov);

    Some(Box::new(
        Sprite2dDrawable{
            texture: sprite.get_texture(),
            x: screen_x,
            z_index: distance,
            brightness: calculate_brightness(distance),
            vertical_offset: sprite.get_vertical_offset(),
            size: sprite.get_size()/distance,
    }))
}

pub fn draw_sprites(camera: &Camera, sprites: &[&dyn Sprite2D]) -> Vec<Box<dyn Drawable>> {
    let camera_pos = camera.position;
    let camera_look = camera.look.normalize_or_zero();

    sprites
        .into_iter()
        .filter_map(|sprite| sprite_to_drawable(camera_pos, camera_look, *sprite)) 
        .collect()
}