use std::{f32::consts::PI, fmt::Debug, hint, rc::Rc};

use macroquad::{color::{Color, RED}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, texture::{self, draw_texture_ex}};

use crate::{constants::{FOV, HORIZONTAL_WALL_SEGEMENTS, VIEW_DISTANCE}, math::find_intersection, model::{Player, Texture, Wall}, texture_manager::{self, TextureManager}};

#[derive(Debug, Clone, Copy)]
pub struct Camera{
    pub position: Vec2,
    pub look: Vec2
}
impl Camera {
    pub fn for_player(player: &Player) -> Self {
        Self {
            position: player.entity.position,
            look: player.entity.position + player.look*VIEW_DISTANCE
        }
    }
}

pub struct Drawable {
    pub z_index: f32,
    pub draw: Box<dyn Fn((f32, f32), Rc<TextureManager>)>
}
impl Debug for Drawable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Drawable").field("z_index", &self.z_index).finish()
    }
}

#[derive(Debug)]
struct RayHit{
    distance_to_ray: f32,
    texture: Texture,
    relative_position: f32, // 0.0 start of wall, 1.0 end of wall
    angle: f32
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

fn calculate_normal_angle(ray_direction: Vec2, ray_origin: Vec2, wall_start: Vec2, wall_end: Vec2) -> f32 {
    let wall_dir = wall_end - wall_start;
    let wall_normal = vec2(-wall_dir.y, wall_dir.x).normalize();
    let dot_product = (ray_direction - ray_origin).normalize_or_zero().dot(wall_normal);
    let clamped_dot = dot_product.clamp(-1.0, 1.0);

    clamped_dot.acos()
}

fn cast_ray(ray_origin: Vec2, ray_direction: Vec2, walls: &[Wall]) -> Option<RayHit> {
    let intesections: Vec<_> = walls.iter()
        .map(|wall| 
            (wall, find_intersection(ray_origin, ray_direction, wall.start, wall.end))
        )
        .filter_map(|(wall, point)| {
            let hit = point?;

            Some(RayHit{
                distance_to_ray: hit.distance(ray_origin),
                texture: wall.texture,
                relative_position: calculate_relative_position(hit, wall.start, wall.end),
                angle: calculate_normal_angle(ray_direction, ray_origin, wall.start, wall.end) 
            })
        })
        .collect();

    intesections.into_iter()
    .min_by(|a, b| a.distance_to_ray.total_cmp(&b.distance_to_ray))
}


fn draw_wall(screen_size: (f32, f32), texture_manager: Rc<TextureManager>, x: usize, distance: f32, angle: f32, texture: Texture, relative_position: f32 ) {
    let height = 1.0/distance;
    let width = 1.0 / HORIZONTAL_WALL_SEGEMENTS as f32;
    let center_x = x as f32 / HORIZONTAL_WALL_SEGEMENTS as f32 + width/2.0;
    let center_y = 0.5;
    let half_pi = PI/2.0;

    let brightness = ((angle-half_pi).abs()/PI).sqrt().clamp(0.3, 1.0);   
    let texture_2d = texture_manager.get_texture(texture);
    let x = center_x - width/2.0;
    let y = center_y - height/2.0;

    let source = Rect::new(
        texture_2d.width() * relative_position,
        0.0,
        texture_2d.width() / HORIZONTAL_WALL_SEGEMENTS as f32,
        texture_2d.height()
    );

    let params = texture::DrawTextureParams { 
        dest_size: Some(vec2(width*screen_size.0, height*screen_size.1)),
        source: Some(source),
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None 
    };
    draw_texture_ex(texture_2d, x*screen_size.0, y*screen_size.1, Color::new(brightness, brightness, brightness, 1.0), params);

    //DEBUG:
    //let base_color = RED;
    //let draw_color = Color::new(base_color.r * brightness, base_color.g * brightness, base_color.b * brightness, 1.0);
    //let x = center_x - width/2.0;
    //let y = center_y - height/2.0;
    //draw_rectangle(
    //    x * screen_size.0,
    //    y * screen_size.1,
    //    width * screen_size.0,
    //    height * screen_size.1,
    //    draw_color
    //);
}

pub fn draw_walls(camera: &Camera, walls: &[Wall]) -> Vec<Drawable> {
    create_rays(camera.look)
        .iter()
        .enumerate()
        .filter_map(|(x, ray)| {
            let hit = cast_ray(camera.position, *ray, walls)?;
            
            let distance = hit.distance_to_ray;
            let angle = hit.angle;
            let texture = hit.texture;
            let relative_position = hit.relative_position;

            let draw_fn = move |screen, texture_manager| {
                draw_wall(screen, texture_manager, x, distance, angle, texture, relative_position);
            };

            Some(Drawable{
                z_index: distance,
                draw: Box::new(draw_fn),
            })
        })
        .collect()
}
