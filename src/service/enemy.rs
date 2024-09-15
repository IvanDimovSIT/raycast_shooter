use macroquad::math::Vec2;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    constants::{ENEMY_MAX_CHASE_DISTANCE, ENEMY_MOVE_SPEED, MOVE_SPEED},
    math::{check_circles_collide, find_intersection, line_intersects_circle},
    model::{melee_enemy::MeleeEnemy, Enemy, Entity, GameEvent, Player, Wall},
};

fn get_enemy_intersecting_walls<'a>(entity: &'a Entity, walls: &'a [Wall]) -> Vec<&'a Wall> {
    walls
        .iter()
        .filter(|wall| line_intersects_circle(wall.start, wall.end, entity.position, entity.size))
        .collect()
}

pub fn enemy_can_attack_player(enemy_entity: &Entity, enemy_attack_range: f32, player: &Player, walls: &[Wall]) -> bool {
    check_circles_collide(
        player.entity.position,
        player.entity.size,
        enemy_entity.position,
        enemy_attack_range,
    ) && !walls.iter().any(|wall| {
        find_intersection(
            enemy_entity.position,
            player.entity.position,
            wall.start,
            wall.end,
        )
        .is_some()
    })
}

fn move_enemy_to_sides(
    player: &Player,
    enemy_entity: Entity,
    walls: &[Wall],
    hit_wall_directions: Vec<Vec2>,
) -> Entity {
    let move_to = hit_wall_directions
        .iter()
        .map(|dir| enemy_entity.position + *dir)
        .filter(|new_pos| {
            get_enemy_intersecting_walls(
                &Entity {
                    position: *new_pos,
                    ..enemy_entity
                },
                walls,
            )
            .is_empty()
        })
        .map(|new_pos| (new_pos, new_pos.distance(player.entity.position)))
        .min_by(|a, b| a.1.total_cmp(&b.1));

    if let Some(new_pos) = move_to {
        Entity {
            position: new_pos.0,
            ..enemy_entity
        }
    } else {
        enemy_entity
    }
}

pub fn move_enemy(player: &Player, enemy_entity: Entity, walls: &[Wall], delta: f32) -> Entity {
    let vector_towards_player = player.entity.position - enemy_entity.position;
    if vector_towards_player.length() > ENEMY_MAX_CHASE_DISTANCE {
        return enemy_entity.clone();
    };

    let move_vector = vector_towards_player.normalize_or_zero() * ENEMY_MOVE_SPEED * delta;
    let new_position = enemy_entity.position + move_vector;

    let new_entity = Entity {
        position: new_position,
        size: enemy_entity.size,
    };
    let intersections = get_enemy_intersecting_walls(&new_entity, walls);
    if intersections.is_empty() {
        return new_entity;
    }

    let wall_directions = intersections
        .iter()
        .map(|w| (w.start - w.end).normalize_or_zero() * delta * MOVE_SPEED)
        .collect();

    move_enemy_to_sides(player, enemy_entity, walls, wall_directions)
}

pub fn update_enemies(player: &Player,
    enemies: Vec<Box<dyn Enemy>>,
    walls: &[Wall],
    delta: f32) -> (Vec<Box<dyn Enemy>>, Vec<GameEvent>) {
    
    let result: (Vec<_>, Vec<_>) = enemies.into_iter()
        .map(|enemy| enemy.update(player, walls, delta))
        .unzip();

    (
        result.0,
        result.1
        .into_iter()
        .flatten()
        .collect()
    )
}

// pub fn move_enemies_towards_player(
//     player: &Player,
//     enemies: Vec<MeleeEnemy>,
//     walls: &[Wall],
//     delta: f32,
// ) -> Vec<MeleeEnemy> {
//     enemies
//         .into_par_iter()
//         .map(|enemy| move_enemy(player, enemy, walls, delta))
//         .collect()
// }

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;

    use crate::{
        constants::{ENEMY_SIZE, PLAYER_SIZE},
        model::Texture,
    };

    use super::*;

    #[test]
    fn test_move_enemy() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: PLAYER_SIZE,
            },
            look: vec2(0.0, 0.0),
        };

        let enemy = Entity {
                position: vec2(10.0, 0.0),
                size: ENEMY_SIZE,
            };

        let walls = vec![Wall {
            texture: Texture::Debug,
            start: vec2(-5.0, 0.0),
            end: vec2(5.0, 0.0),
        }];

        let delta = 1.0;

        let moved_enemy = move_enemy(&player, enemy.clone(), &walls, delta);

        assert!(
            moved_enemy.position.distance(player.entity.position)
                < enemy.position.distance(player.entity.position)
        );

        let far_enemy = Entity {
                position: vec2(ENEMY_MAX_CHASE_DISTANCE + 10.0, 0.0),
                size: ENEMY_SIZE,
            };

        let not_moved_enemy = move_enemy(&player, far_enemy.clone(), &walls, delta);
        assert_eq!(not_moved_enemy.position, far_enemy.position);
    }
}
