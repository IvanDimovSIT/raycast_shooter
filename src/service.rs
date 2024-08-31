use macroquad::math::Vec2;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    constants::{
        ENEMY_MAX_CHASE_DISTANCE, ENEMY_MOVE_SPEED, GUN_DPS, MAX_SHOOT_DISTANCE, MOVE_SPEED,
    },
    math::{check_circles_collide, find_intersection, line_intersects_circle},
    model::{enemy::Enemy, key_object::KeyObject, Entity, GameEvent, Player, PlayerInfo, Wall},
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
            ..player_entity
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

fn is_enemy_intersecting_walls<'a>(entity: &'a Entity, walls: &'a [Wall]) -> Option<&'a Wall> {
    walls
        .iter()
        .find(|wall| line_intersects_circle(wall.start, wall.end, entity.position, entity.size))
}

fn move_enemy_to_sides(
    player: &Player,
    enemy: Enemy,
    walls: &[Wall],
    hit_wall_direction: Vec2,
) -> Enemy {
    let left_pos = enemy.entity.position + hit_wall_direction;
    let left_dist = left_pos.distance(player.entity.position);
    let right_pos = enemy.entity.position - hit_wall_direction;
    let right_dist = right_pos.distance(player.entity.position);

    let move_pos = if left_dist < right_dist {
        left_pos
    } else {
        right_pos
    };

    let new_entity = Entity {
        position: move_pos,
        size: enemy.entity.size,
    };

    if is_enemy_intersecting_walls(&new_entity, walls).is_none() {
        Enemy {
            entity: new_entity,
            ..enemy
        }
    } else {
        enemy
    }
}

fn move_enemy(player: &Player, enemy: Enemy, walls: &[Wall], delta: f32) -> Enemy {
    let vector_towards_player = player.entity.position - enemy.entity.position;
    if vector_towards_player.length() > ENEMY_MAX_CHASE_DISTANCE {
        return enemy.clone();
    };

    let move_vector = vector_towards_player.normalize_or_zero() * ENEMY_MOVE_SPEED * delta;
    let new_position = enemy.entity.position + move_vector;

    let new_entity = Entity {
        position: new_position,
        size: enemy.entity.size,
    };
    let intersection = is_enemy_intersecting_walls(&new_entity, walls);
    if intersection.is_none() {
        return Enemy {
            entity: new_entity,
            ..enemy
        };
    }
    let hit_wall = intersection.unwrap();
    let wall_direction = (hit_wall.start - hit_wall.end).normalize_or_zero() * delta * MOVE_SPEED;

    move_enemy_to_sides(player, enemy, walls, wall_direction)
}

pub fn move_enemies_towards_player(
    player: &Player,
    enemies: Vec<Enemy>,
    walls: &[Wall],
    delta: f32,
) -> Vec<Enemy> {
    enemies
        .into_par_iter()
        .map(|enemy| move_enemy(player, enemy, walls, delta))
        .collect()
}

fn find_shot_enemy<'a>(
    player: &'a Player,
    enemies: &'a [Enemy],
    walls: &'a [Wall],
) -> Option<&'a Enemy> {
    let shoot_ray = player.look.normalize_or_zero() * MAX_SHOOT_DISTANCE;

    let closest_hit_wall = walls
        .iter()
        .filter_map(|wall| {
            find_intersection(player.entity.position, shoot_ray, wall.start, wall.end)
        })
        .map(|p| p.distance(player.entity.position))
        .min_by(|a, b| a.total_cmp(b));

    let max_distance = if let Some(some) = closest_hit_wall {
        some
    } else {
        MAX_SHOOT_DISTANCE
    };

    let hit_enemy = enemies
        .iter()
        .filter(|enemy| {
            line_intersects_circle(
                player.entity.position,
                shoot_ray,
                enemy.entity.position,
                enemy.entity.size,
            )
        })
        .filter_map(|enemy| {
            let dist = enemy.entity.position.distance(player.entity.position);
            if dist > max_distance {
                None
            } else {
                Some((enemy, dist))
            }
        })
        .min_by(|a, b| a.1.total_cmp(&b.1));

    Some(hit_enemy?.0)
}

pub fn shoot_enemies(
    player: &Player,
    info: &PlayerInfo,
    enemies: Vec<Enemy>,
    walls: &[Wall],
    delta: f32,
) -> (Vec<Enemy>, Vec<GameEvent>) {
    if !info.is_shooting {
        return (enemies, vec![]);
    }

    let shot_enemy_option = find_shot_enemy(player, &enemies, walls);
    if shot_enemy_option.is_none() {
        return (enemies, vec![]);
    }
    let shot_enemy_id = shot_enemy_option.unwrap().id;
    let damage = delta * GUN_DPS;

    let new_hp_enemies: Vec<_> = enemies
        .into_iter()
        .map(|enemy| {
            if enemy.id == shot_enemy_id {
                Enemy {
                    hp: enemy.hp - damage,
                    ..enemy
                }
            } else {
                enemy
            }
        })
        .collect();

    let game_events = new_hp_enemies
        .iter()
        .filter_map(|enemy| {
            if enemy.hp <= 0.0 {
                Some(GameEvent::EnemyKilled {
                    position: enemy.entity.position,
                })
            } else {
                None
            }
        })
        .collect();

    let surviving_enemies = new_hp_enemies
        .into_iter()
        .filter(|enemy| enemy.hp > 0.0)
        .collect();

    (surviving_enemies, game_events)
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
