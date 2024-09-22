use macroquad::math::{vec2, Vec2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    constants::{MOVE_SPEED, PLAYER_MAX_HEALTH, PLAYER_REGENERATION},
    math::{check_circles_collide, line_intersects_circle, rotate_point},
    model::{key_object::KeyObject, Entity, GameEvent, GameObjects, Player, PlayerInfo, Wall},
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

pub fn regenerate_health(player_health: f32, delta: f32) -> f32 {
    (player_health + delta * PLAYER_REGENERATION).min(PLAYER_MAX_HEALTH)
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;
    use uuid::Uuid;

    use crate::model::Texture;

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
