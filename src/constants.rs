use std::f32::consts::PI;

use macroquad::{
    color::Color,
    color_u8,
    input::KeyCode,
    math::{vec2, Vec2},
};

pub const FOCUS_KEY: KeyCode = KeyCode::Escape;

pub const LEVEL_PATH: &str = "resources/level.json";
pub const TEXTURE_PATH: &str = "resources/textures/";

pub const HORIZONTAL_WALL_SEGEMENTS: u32 = 200;
pub const FOV: f32 = PI / 4.0;
pub const VIEW_DISTANCE: f32 = 1000.0;
pub const WALL_RESOLUTION: f32 = 10000.0;

pub const PLAYER_MAX_HEALTH: f32 = 100.0;
pub const PLAYER_REGENERATION: f32 = 2.0;
pub const PLAYER_SIZE: f32 = 0.2;
pub const KEYS_TURN_SPEED: f32 = 1.3;
pub const MOUSE_TURN_SPEED: f32 = 50.0;
pub const MOVE_SPEED: f32 = 1.9;

pub const FLOOR_COLOR: Color = color_u8!(55, 55, 75, 255);
pub const CEILING_COLOR: Color = color_u8!(60, 40, 40, 255);

pub const MIN_BRIGHTNESS: f32 = 0.2;

pub const DEBUG_DRAW_DELAY_MS: u64 = 50;
pub const DEBUG_INITAL_DRAW_DELAY_MS: u64 = 500;
pub const ENTER_DEBUG_MODE_KEY: KeyCode = KeyCode::GraveAccent;
pub const EXIT_DEBUG_MODE_KEY: KeyCode = KeyCode::Delete;

pub const KEY_SIZE: f32 = 0.5;
pub const KEY_ANIMATION_SPEED_MOVEMENT: u128 = 3000;
pub const KEY_ANIMATION_SPEED_TEXTURES: u128 = 800;
pub const KEY_DRAW_SIZE_MOD: f32 = 0.5;

pub const GUN_FIRE_ANIMATION_SPEED: u128 = 80;
pub const GUN_POSITION: Vec2 = vec2(0.6, 0.6);
pub const GUN_ROTATION: f32 = 0.5;
pub const GUN_SIZE: f32 = 0.7;

pub const ENEMY_HEIGHT_CHANGE_ANIMATION_SPEED: u128 = 300;
pub const ENEMY_BASE_HEIGHT_OFFSET: f32 = -0.2;
pub const ENEMY_HEIGHT_AMPLITUDE: f32 = 0.04;
pub const ENEMY_SIZE: f32 = 0.8;
pub const ENEMY_HP: f32 = 100.0;
pub const ENEMY_MAX_CHASE_DISTANCE: f32 = 40.0;

pub const MELEE_ENEMY_ANIMATION_SPEED: u128 = 100;
pub const MELEE_ENEMY_MOVE_SPEED: f32 = 1.9;
pub const MELEE_ENEMY_ATTACK_RANGE: f32 = 0.1;
pub const MELEE_ENEMY_ATTACK_DELAY: f32 = 1.0;
pub const MELEE_ENEMY_DAMAGE: f32 = 80.0;

pub const RANGED_ENEMY_ANIMATION_SPEED: u128 = 100;
pub const RANGED_ENEMY_SHOOT_RANGE: f32 = 8.0;
pub const RANGED_ENEMY_MOVE_SPEED: f32 = 1.7;
pub const RANGED_ENEMY_ATTACK_DELAY: f32 = 1.5;
pub const RANGED_ENEMY_DAMAGE: f32 = 40.0;
pub const RANGED_ENEMY_SHOT_SIZE: f32 = 0.1;
pub const RANGED_ENEMY_SHOT_SPEED: f32 = 3.4;

pub const RELOAD_SPEED: f32 = 3.0;
pub const SHOOT_SPEED: f32 = 0.1;
pub const MAX_BULLETS: usize = 30;
pub const GUN_DAMAGE: f32 = 50.0;
pub const MAX_SHOOT_DISTANCE: f32 = 100.0;

pub const GUNSHOT_ANIMATION_LENGTH: f32 = 0.3;
pub const GUNSHOT_ANIMATION_SPEED: u128 = 80;
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
pub const KEYS_UI_POSITION: Vec2 = vec2(0.05, 0.84);
pub const KEYS_UI_BOX_WIDTH_TO_HEIGHT: f32 = 5.0;
pub const KEYS_UI_FIND_EXIT_TEXT: &str = "Find the exit";

pub const BULLETS_UI_BOX_COLOR: Color = Color::new(0.1, 0.1, 0.1, 0.5);
pub const BULLETS_UI_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.7);
pub const BULLETS_UI_SIZE: f32 = 0.06;
pub const BULLETS_UI_POSITION: Vec2 = vec2(0.05, 0.91);
pub const BULLETS_UI_BOX_WIDTH_TO_HEIGHT: f32 = 5.0;

pub const HEALTH_DISPLAY_POSITION: Vec2 = vec2(0.05, 0.95);
pub const HEALTH_DISPLAY_BORDER_SIZE: f32 = 0.005;
pub const HEALTH_DISPLAY_HEIGHT: f32 = 0.03;
pub const HEALTH_DISPLAY_WIDTH: f32 = 0.14;
pub const HEALTH_DISPLAY_BACKGROUND_COLOR: Color = Color::new(0.1, 0.1, 0.1, 1.0);
pub const HEALTH_DISPLAY_COLOR: Color = Color::new(0.2, 0.9, 0.2, 1.0);

pub const PROJECTILE_OFFSET: f32 = 0.0;
