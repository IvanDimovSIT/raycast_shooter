use std::f32::consts::PI;

use macroquad::{
    color::Color,
    color_u8,
    input::KeyCode,
    math::{vec2, Vec2},
};

pub const HORIZONTAL_WALL_SEGEMENTS: u32 = 150;
pub const FOV: f32 = PI / 4.0;
pub const VIEW_DISTANCE: f32 = 1000.0;
pub const WALL_RESOLUTION: f32 = 10000.0;

pub const TURN_SPEED: f32 = 1.0;
pub const MOVE_SPEED: f32 = 1.0;

pub const FLOOR_COLOR: Color = color_u8!(55, 55, 75, 255);
pub const CEILING_COLOR: Color = color_u8!(60, 40, 40, 255);

pub const MIN_BRIGHTNESS: f32 = 0.2;

pub const DEBUG_DRAW_DELAY_MS: u64 = 50;
pub const DEBUG_INITAL_DRAW_DELAY_MS: u64 = 500;
pub const ENTER_DEBUG_MODE_KEY: KeyCode = KeyCode::GraveAccent;
pub const EXIT_DEBUG_MODE_KEY: KeyCode = KeyCode::Escape;

pub const KEY_ANIMATION_SPEED_MOVEMENT: u128 = 3000;
pub const KEY_ANIMATION_SPEED_TEXTURES: u128 = 800;
pub const KEY_DRAW_SIZE_MOD: f32 = 0.5;

pub const GUN_FIRE_ANIMATION_SPEED: u128 = 80;
pub const GUN_POSITION: Vec2 = vec2(0.6, 0.6);
pub const GUN_ROTATION: f32 = 0.5;
pub const GUN_SIZE: f32 = 0.7;

pub const ENEMY_ANIMATION_SPEED: u128 = 100;
pub const ENEMY_DRAW_SIZE_MOD: f32 = 5.0;

pub const ENEMY_MAX_CHASE_DISTANCE: f32 = 100.0;
pub const ENEMY_MOVE_SPEED: f32 = 0.6;
