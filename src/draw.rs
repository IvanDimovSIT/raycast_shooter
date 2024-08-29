use std::{f32::consts::PI, fmt::Debug};

use macroquad::{
    color::Color,
    math::{vec2, Rect, Vec2},
    texture::{self, draw_texture_ex},
};
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    constants::{FOV, HORIZONTAL_WALL_SEGEMENTS, MIN_BRIGHTNESS, VIEW_DISTANCE},
    math::find_intersection,
    model::{Player, Texture, Wall},
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
            let brightness = (1.0 / distance.max(0.001))
                .sqrt()
                .clamp(MIN_BRIGHTNESS, 1.0);
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
