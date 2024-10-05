use std::{error::Error, fs::read};

use macroquad::math::{vec2, Vec2};
use serde::Deserialize;
use serde_json::from_slice;

use crate::{
    constants::{KEY_SIZE, LEVEL_PATH, PLAYER_SIZE},
    model::{enemy::EnemyType, key_object::KeyObject, Entity, GameObjects, PlayerInfo, Texture},
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
struct ExitTigger {
    position: [f32; 2],
    size: f32,
}

#[derive(Deserialize)]
struct Enemy {
    position: [f32; 2],
    enemy_type: EnemyType,
}
impl Into<crate::model::enemy::Enemy> for Enemy {
    fn into(self) -> crate::model::enemy::Enemy {
        self.enemy_type.to_enemy(array_to_vec(self.position))
    }
}

#[derive(Deserialize)]
struct Level {
    walls: Vec<Wall>,
    player: Player,
    enemies: Vec<Enemy>,
    keys: Vec<[f32; 2]>,
    exit_triggers: Vec<ExitTigger>,
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

        let enemies = self.enemies.into_iter().map(|enemy| enemy.into()).collect();

        let keys: Vec<_> = self
            .keys
            .iter()
            .map(|key| KeyObject {
                entity: Entity {
                    position: array_to_vec(*key),
                    size: KEY_SIZE,
                },
            })
            .collect();

        if keys.is_empty() {
            panic!("Invalid level: no keys");
        }

        let exit_triggers: Vec<_> = self
            .exit_triggers
            .iter()
            .map(|exit_tigger| Entity {
                position: array_to_vec(exit_tigger.position),
                size: exit_tigger.size,
            })
            .collect();

        if exit_triggers.is_empty() {
            panic!("Invalid level: no exit triggers");
        }

        GameObjects {
            player,
            player_info: PlayerInfo::default(),
            walls,
            enemies,
            keys,
            exit_triggers,
            decorations: vec![],
            projectiles: vec![],
        }
    }
}

fn array_to_vec(arr: [f32; 2]) -> Vec2 {
    vec2(arr[0], arr[1])
}

pub fn load_level(level_number: u32) -> Result<GameObjects, Box<dyn Error>> {
    let level_path = format!("{LEVEL_PATH}level{level_number}.json");

    println!("Loading level: {}", level_path);

    let data = read(&level_path)?;

    let level: Level = from_slice(&data).expect("Failed to deserialize level data");

    Ok(level.into())
}
