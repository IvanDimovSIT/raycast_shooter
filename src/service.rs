use macroquad::math::Vec2;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    constants::{ENEMY_MAX_CHASE_DISTANCE, ENEMY_MOVE_SPEED}, draw::sprite_2d::Sprite2D, math::{check_circles_collide, line_intersects_circle}, model::{enemy::Enemy, key_object::KeyObject, Entity, GameEvent, Player, Wall}
};

pub fn move_player(player_entity: Entity, movement: Vec2, walls: &[Wall]) -> Entity {
    let new_pos = player_entity.position + movement;

    let is_collision = walls
        .par_iter()
        .any(|wall| line_intersects_circle(wall.start, wall.end, new_pos, player_entity.size));

    if is_collision {
        player_entity
    } else {
        Entity {
            position: new_pos,
            size: player_entity.size,
        }
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

fn is_enemy_intersecting_walls(entity: &Entity, walls: &[Wall]) -> bool {
    walls
        .iter()
        .any(|wall| line_intersects_circle(wall.start, wall.end, entity.position, entity.size))
}


fn move_enemy(player: &Player, enemy: Enemy, walls: &[Wall], delta: f32) -> Enemy {
    let vector_towards_player = player.entity.position - enemy.entity.position;
    if vector_towards_player.length() > ENEMY_MAX_CHASE_DISTANCE {
        return enemy.clone();
    };

    let move_vector = vector_towards_player.normalize_or_zero() * ENEMY_MOVE_SPEED * delta;
    let new_position = enemy.entity.position + move_vector;
    let is_free_to_move = !is_enemy_intersecting_walls(&enemy.entity, walls);
    if is_free_to_move {
        return Enemy{ id: enemy.id, entity: Entity { position: new_position, size: enemy.entity.size }, hp: enemy.hp, textures: enemy.textures };
    }

    println!("Unimplemented pathfinding!");

    enemy
}

pub fn move_enemies_towards_player(player: &Player, enemies: Vec<Enemy>, walls: &[Wall], delta: f32) -> Vec<Enemy> {
    enemies
        .into_par_iter()
        .map(|enemy| move_enemy(player, enemy, walls, delta))
        .collect()
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;

    use crate::model::Texture;

    use super::*;

    #[test]
    fn test_move_entity() {
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

        let moved1 = move_player(entity, movement1, &walls);
        assert_eq!(moved1.position, entity.position);

        let movement2 = vec2(0.0, 0.1);
        let moved2 = move_player(entity, movement2, &walls);

        assert!(moved2.position.y > entity.position.y);
    }
}
