use std::fs::read;

use macroquad::math::{vec2, Vec2};
use serde::Deserialize;
use serde_json::from_slice;

use crate::{
    constants::{ENEMY_SIZE, KEY_SIZE, LEVEL_PATH, PLAYER_SIZE},
    model::{enemy::Enemy, key_object::KeyObject, Entity, GameObjects, PlayerInfo, Texture},
};

#[derive(Deserialize)]
struct Wall {
    start: [f32; 2],
    end: [f32; 2],
    #[serde(default)]
    texture: Texture,
}

#[derive(Deserialize)]
struct Player {
    position: [f32; 2],
    look: [f32; 2],
}

#[derive(Deserialize)]
struct Level {
    walls: Vec<Wall>,
    player: Player,
    enemies: Vec<[f32; 2]>,
    keys: Vec<[f32; 2]>,
}
impl Into<GameObjects> for Level {
    fn into(self) -> GameObjects {
        let player = crate::model::Player {
            entity: Entity {
                position: array_to_vec(self.player.position),
                size: PLAYER_SIZE,
            },
            look: array_to_vec(self.player.look).normalize_or_zero(),
        };

        let walls = self
            .walls
            .iter()
            .map(|wall| crate::model::Wall {
                texture: wall.texture,
                start: array_to_vec(wall.start),
                end: array_to_vec(wall.end),
            })
            .collect();

        let enemies = self
            .enemies
            .iter()
            .map(|enemy| Enemy {
                entity: Entity {
                    position: array_to_vec(*enemy),
                    size: ENEMY_SIZE,
                },
                ..Default::default()
            })
            .collect();

        let keys = self
            .keys
            .iter()
            .map(|key| KeyObject {
                entity: Entity {
                    position: array_to_vec(*key),
                    size: KEY_SIZE,
                },
                ..Default::default()
            })
            .collect();

        GameObjects {
            player,
            player_info: PlayerInfo::default(),
            walls,
            enemies,
            keys,
            decorations: vec![],
        }
    }
}

fn array_to_vec(arr: [f32; 2]) -> Vec2 {
    vec2(arr[0], arr[1])
}

pub fn load_level() -> GameObjects {
    let data =
        read(LEVEL_PATH).unwrap_or_else(|_| panic!("Failed to find level file '{}'", LEVEL_PATH));
    let level: Level = from_slice(&data).expect("Failed to deserialize level data");

    level.into()
}
