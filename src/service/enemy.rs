use macroquad::math::Vec2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    constants::{ENEMY_MAX_CHASE_DISTANCE, ENEMY_MOVE_SPEED, MOVE_SPEED},
    math::line_intersects_circle,
    model::{enemy::Enemy, Entity, Player, Wall},
};

fn get_enemy_intersecting_walls<'a>(entity: &'a Entity, walls: &'a [Wall]) -> Vec<&'a Wall> {
    walls
        .iter()
        .filter(|wall| line_intersects_circle(wall.start, wall.end, entity.position, entity.size))
        .collect()
}

fn move_enemy_to_sides(
    player: &Player,
    enemy: Enemy,
    walls: &[Wall],
    hit_wall_directions: Vec<Vec2>,
) -> Enemy {
    let move_to = hit_wall_directions
        .iter()
        .map(|dir| enemy.entity.position + *dir)
        .filter(|new_pos| {
            get_enemy_intersecting_walls(
                &Entity {
                    position: *new_pos,
                    ..enemy.entity
                },
                walls,
            )
            .is_empty()
        })
        .map(|new_pos| (new_pos, new_pos.distance(player.entity.position)))
        .min_by(|a, b| a.1.total_cmp(&b.1));

    if let Some(new_pos) = move_to {
        Enemy {
            entity: Entity {
                position: new_pos.0,
                ..enemy.entity
            },
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
    let intersections = get_enemy_intersecting_walls(&new_entity, walls);
    if intersections.is_empty() {
        return Enemy {
            entity: new_entity,
            ..enemy
        };
    }

    let wall_directions = intersections
        .iter()
        .map(|w| (w.start - w.end).normalize_or_zero() * delta * MOVE_SPEED)
        .collect();

    move_enemy_to_sides(player, enemy, walls, wall_directions)
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

        let enemy = Enemy {
            entity: Entity {
                position: vec2(10.0, 0.0),
                size: ENEMY_SIZE,
            },
            hp: 100.0,
            ..Default::default()
        };

        let walls = vec![Wall {
            texture: Texture::Debug,
            start: vec2(-5.0, 0.0),
            end: vec2(5.0, 0.0),
        }];

        let delta = 1.0;

        let moved_enemy = move_enemy(&player, enemy.clone(), &walls, delta);

        assert!(
            moved_enemy.entity.position.distance(player.entity.position)
                < enemy.entity.position.distance(player.entity.position)
        );

        let far_enemy = Enemy {
            entity: Entity {
                position: vec2(ENEMY_MAX_CHASE_DISTANCE + 10.0, 0.0),
                size: ENEMY_SIZE,
            },
            ..Default::default()
        };

        let not_moved_enemy = move_enemy(&player, far_enemy.clone(), &walls, delta);
        assert_eq!(not_moved_enemy.entity.position, far_enemy.entity.position);
    }
}
