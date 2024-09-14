use macroquad::math::{vec2, Vec2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    constants::{ENEMY_ATTACK_RANGE, ENEMY_DPS, MOVE_SPEED},
    math::{check_circles_collide, find_intersection, line_intersects_circle, rotate_point},
    model::{
        enemy::Enemy, key_object::KeyObject, Entity, GameEvent, GameObjects, Player, PlayerInfo,
        Wall,
    },
};

pub fn move_player_entity(player_entity: Entity, movement: Vec2, walls: &[Wall]) -> Entity {
    let new_pos = player_entity.position + movement;

    let is_collision = walls
        .par_iter()
        .any(|wall| line_intersects_circle(wall.start, wall.end, new_pos, player_entity.size));

    if is_collision {
        player_entity
    } else {
        Entity {
            position: new_pos,
            ..player_entity
        }
    }
}

pub fn move_player(
    game_objects: &GameObjects,
    player: Player,
    direction: Vec2,
    delta: f32,
) -> Player {
    Player {
        entity: move_player_entity(
            player.entity,
            direction * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        ..player
    }
}

pub fn check_pickup_key(player: &Player, keys: &[KeyObject]) -> Vec<GameEvent> {
    keys.iter()
        .filter_map(|key| {
            if check_circles_collide(
                key.entity.position,
                key.entity.size,
                player.entity.position,
                player.entity.size,
            ) {
                Some(GameEvent::PickUpKey(key.id))
            } else {
                None
            }
        })
        .collect()
}

pub fn turn_player(player: Player, thetha: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), thetha);
    Player { look, ..player }
}

fn enemy_can_attack_player(enemy: &Enemy, player: &Player, walls: &[Wall]) -> bool {
    check_circles_collide(
        player.entity.position,
        player.entity.size,
        enemy.entity.position,
        ENEMY_ATTACK_RANGE,
    ) && !walls.iter().any(|wall| {
        find_intersection(
            enemy.entity.position,
            player.entity.position,
            wall.start,
            wall.end,
        )
        .is_some()
    })
}

pub fn deal_damage_to_player(game_objects: &GameObjects, delta: f32) -> PlayerInfo {
    let damage: f32 = game_objects
        .enemies
        .iter()
        .map(|enemy| {
            if enemy_can_attack_player(enemy, &game_objects.player, &game_objects.walls) {
                delta * ENEMY_DPS
            } else {
                0.0
            }
        })
        .sum();

    PlayerInfo {
        health: game_objects.player_info.health - damage,
        ..game_objects.player_info
    }
}

pub fn is_player_at_exit(game_objects: &GameObjects) -> bool {
    game_objects.exit_triggers.iter().any(|trigger| {
        check_circles_collide(
            trigger.position,
            trigger.size,
            game_objects.player.entity.position,
            game_objects.player.entity.size,
        )
    })
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;
    use uuid::Uuid;

    use crate::{
        constants::{ENEMY_SIZE, PLAYER_SIZE},
        model::Texture,
    };

    use super::*;

    #[test]
    fn test_move_player_entity() {
        let entity = Entity {
            position: vec2(0.0, 0.0),
            size: 1.0,
        };
        let movement1 = vec2(0.0, 1.0);
        let walls = vec![Wall {
            texture: Texture::Debug,
            start: vec2(-10.0, 1.5),
            end: vec2(10.0, 1.5),
        }];

        let moved1 = move_player_entity(entity, movement1, &walls);
        assert_eq!(moved1.position, entity.position);

        let movement2 = vec2(0.0, 0.1);
        let moved2 = move_player_entity(entity, movement2, &walls);

        assert!(moved2.position.y > entity.position.y);
    }

    #[test]
    fn test_enemy_can_attack_player() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: PLAYER_SIZE,
            },
            look: vec2(0.0, 0.0),
        };

        let enemy = Enemy {
            entity: Entity {
                position: vec2(0.0, ENEMY_ATTACK_RANGE + PLAYER_SIZE + 0.1),
                size: ENEMY_SIZE,
            },
            ..Default::default()
        };

        let walls = vec![];

        assert!(!enemy_can_attack_player(&enemy, &player, &walls));

        let enemy2 = Enemy {
            entity: Entity {
                position: vec2(0.0, ENEMY_ATTACK_RANGE + PLAYER_SIZE - 0.1),
                size: ENEMY_SIZE,
            },
            ..Default::default()
        };

        assert!(enemy_can_attack_player(&enemy2, &player, &walls));

        let walls2 = vec![Wall {
            texture: Texture::default(),
            start: vec2(-100.0, ENEMY_ATTACK_RANGE / 2.0),
            end: vec2(100.0, ENEMY_ATTACK_RANGE / 2.0),
        }];

        assert!(!enemy_can_attack_player(&enemy, &player, &walls2));
    }

    #[test]
    fn test_check_pickup_key() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: 1.0,
            },
            look: vec2(0.0, 0.0),
        };

        let key = KeyObject {
            id: Uuid::new_v4(),
            entity: Entity {
                position: vec2(0.0, 0.5),
                size: 1.0,
            },
        };

        let keys = vec![key];

        let events = check_pickup_key(&player, &keys);
        assert_eq!(events.len(), 1);

        let player_far = Player {
            entity: Entity {
                position: vec2(10.0, 10.0),
                size: 1.0,
            },
            look: vec2(0.0, 0.0),
        };

        let events_far = check_pickup_key(&player_far, &keys);
        assert_eq!(events_far.len(), 0);
    }
}
