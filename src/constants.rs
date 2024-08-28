use std::f32::consts::PI;

use macroquad::{color::Color, color_u8};

pub const HORIZONTAL_WALL_SEGEMENTS: u32 = 150;
pub const FOV: f32 = PI/4.0;
pub const VIEW_DISTANCE: f32 = 1000.0;

pub const TURN_SPEED: f32 = 1.0;
pub const MOVE_SPEED: f32 = 1.0;

pub const FLOOR_COLOR: Color = color_u8!(55, 55, 75, 255);
pub const CEILING_COLOR: Color = color_u8!(60, 40, 40, 255);

pub const DEBUG_DRAW_DELAY_MS: u64 = 100;