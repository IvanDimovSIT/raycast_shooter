use std::f32::consts::PI;

use macroquad::{
    color::Color,
    color_u8,
    input::KeyCode,
    math::{vec2, Vec2},
};

pub const LEVEL_PATH: &str = "resources/level.json";
pub const TEXTURE_PATH: &str = "resources/textures/";

pub const HORIZONTAL_WALL_SEGEMENTS: u32 = 150;
pub const FOV: f32 = PI / 4.0;
pub const VIEW_DISTANCE: f32 = 1000.0;
pub const WALL_RESOLUTION: f32 = 10000.0;

pub const PLAYER_SIZE: f32 = 0.2;
pub const TURN_SPEED: f32 = 1.3;
pub const MOVE_SPEED: f32 = 1.9;

pub const FLOOR_COLOR: Color = color_u8!(55, 55, 75, 255);
pub const CEILING_COLOR: Color = color_u8!(60, 40, 40, 255);

pub const MIN_BRIGHTNESS: f32 = 0.2;

pub const DEBUG_DRAW_DELAY_MS: u64 = 50;
pub const DEBUG_INITAL_DRAW_DELAY_MS: u64 = 500;
pub const ENTER_DEBUG_MODE_KEY: KeyCode = KeyCode::GraveAccent;
pub const EXIT_DEBUG_MODE_KEY: KeyCode = KeyCode::Escape;

pub const KEY_SIZE: f32 = 0.5;
pub const KEY_ANIMATION_SPEED_MOVEMENT: u128 = 3000;
pub const KEY_ANIMATION_SPEED_TEXTURES: u128 = 800;
pub const KEY_DRAW_SIZE_MOD: f32 = 0.5;

pub const GUN_FIRE_ANIMATION_SPEED: u128 = 80;
pub const GUN_POSITION: Vec2 = vec2(0.6, 0.6);
pub const GUN_ROTATION: f32 = 0.5;
pub const GUN_SIZE: f32 = 0.7;

pub const ENEMY_ANIMATION_SPEED: u128 = 100;
pub const ENEMY_SIZE: f32 = 0.2;
pub const ENEMY_DRAW_SIZE_MOD: f32 = 9.0;
pub const ENEMY_HP: f32 = 10.0;
pub const ENEMY_MAX_CHASE_DISTANCE: f32 = 40.0;
pub const ENEMY_MOVE_SPEED: f32 = 1.8;
pub const ENEMY_DPS: f32 = 80.0;
pub const ENEMY_ATTACK_RANGE: f32 = 0.6;

pub const GUN_DPS: f32 = 50.0;
pub const MAX_SHOOT_DISTANCE: f32 = 100.0;

pub const GUNSHOT_ANIMATION_LENGTH: f32 = 0.5;
pub const GUNSHOT_ANIMATION_SPEED: u128 = 100;
pub const CREATE_GUNSHOT_ANIMATION_RATE: f32 = 2.5;
pub const CREATE_GUNSHOT_HIT_ANIMATION_OFFSET_TO_CAMERA: f32 = 0.1;

pub const CORPSE_SIZE: f32 = 0.15;
pub const CORPSE_OFFSET: f32 = 0.45;

pub const GAME_OVER_TEXT: &str = "Game Over!";
pub const GAME_OVER_TEXT_SIZE: f32 = 0.15;
pub const TRY_AGAIN_TEXT: &str = "Try again (Y/N)?";
pub const TRY_AGAIN_TEXT_SIZE: f32 = 0.07;

pub const GAME_WON_TEXT: &str = "Escaped!";
pub const GAME_WON_TEXT_SIZE: f32 = 0.15;
pub const GAME_WON_TIME_TEXT_SIZE: f32 = 0.10;
pub const TRY_AGAIN_WON_TEXT: &str = "Play again (Y/N)?";
pub const TRY_AGAIN_WON_TEXT_SIZE: f32 = 0.07;

pub const KEYS_UI_BOX_COLOR: Color = Color::new(0.1, 0.1, 0.1, 0.5);
pub const KEYS_UI_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.7);
pub const KEYS_UI_SIZE: f32 = 0.05;
pub const KEYS_UI_POSITION: Vec2 = vec2(0.05, 0.92);
