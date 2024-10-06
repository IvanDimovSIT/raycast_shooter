use macroquad::math::Vec2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    constants::{ENEMY_MAX_CHASE_DISTANCE, MOVE_SPEED, RANGED_ENEMY_SHOT_SPEED},
    math::{check_circles_collide, find_intersection, line_intersects_circle},
    model::{
        enemy::{Enemy, EnemyType},
        Entity, GameEvent, Player, Wall,
    },
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
        .flat_map(|dir| vec![enemy.entity.position + *dir, enemy.entity.position - *dir])
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
    let speed = enemy.enemy_type.get_movement_speed();

    let move_vector = vector_towards_player.normalize_or_zero() * speed * delta;
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

fn move_enemy_for_type(player: &Player, enemy: Enemy, walls: &[Wall], delta: f32) -> Enemy {
    match &enemy.enemy_type {
        EnemyType::Melee | EnemyType::MeleeSlow => move_enemy(player, enemy, walls, delta),
        EnemyType::Ranged => {
            if !enemy_can_attack_player(&enemy, player, walls) {
                move_enemy(player, enemy, walls, delta)
            } else {
                enemy
            }
        }
    }
}

fn enemy_can_attack_player(enemy: &Enemy, player: &Player, walls: &[Wall]) -> bool {
    check_circles_collide(
        player.entity.position,
        player.entity.size,
        enemy.entity.position,
        enemy.enemy_type.get_attack_range() + enemy.entity.size,
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

fn melee_enemy_attack_player(enemy: Enemy) -> (Enemy, Vec<GameEvent>) {
    (
        Enemy {
            attack_delay: enemy.enemy_type.get_attack_speed(),
            ..enemy
        },
        vec![GameEvent::PlayerTakeDamage(
            enemy.enemy_type.get_attack_damage(),
        )],
    )
}

fn ranged_enemy_attack_player(enemy: Enemy, player: &Player) -> (Enemy, Vec<GameEvent>) {
    let create_projectile_event = GameEvent::CreateProjectile {
        position: enemy.entity.position,
        direction: (player.entity.position - enemy.entity.position).normalize_or_zero()
            * RANGED_ENEMY_SHOT_SPEED,
        damage: enemy.enemy_type.get_attack_damage(),
    };

    (
        Enemy {
            attack_delay: enemy.enemy_type.get_attack_speed(),
            ..enemy
        },
        vec![create_projectile_event],
    )
}

fn enemy_attack_player(
    player: &Player,
    enemy: Enemy,
    walls: &[Wall],
    delta: f32,
) -> (Enemy, Vec<GameEvent>) {
    let new_attack_delay = (enemy.attack_delay - delta).max(0.0);
    let updated_enemy = Enemy {
        attack_delay: new_attack_delay,
        ..enemy
    };

    if new_attack_delay > 0.0 || !enemy_can_attack_player(&updated_enemy, player, walls) {
        return (updated_enemy, vec![]);
    }
    match enemy.enemy_type {
        EnemyType::Melee | EnemyType::MeleeSlow => melee_enemy_attack_player(updated_enemy),
        EnemyType::Ranged => ranged_enemy_attack_player(updated_enemy, player),
    }
}

pub fn enemies_attack_player(
    player: &Player,
    enemies: Vec<Enemy>,
    walls: &[Wall],
    delta: f32,
) -> (Vec<Enemy>, Vec<GameEvent>) {
    let (attacked, events): (Vec<_>, Vec<_>) = enemies
        .into_iter()
        .map(|enemy| enemy_attack_player(player, enemy, walls, delta))
        .unzip();

    (attacked, events.into_iter().flatten().collect())
}

pub fn move_enemies_towards_player(
    player: &Player,
    enemies: Vec<Enemy>,
    walls: &[Wall],
    delta: f32,
) -> Vec<Enemy> {
    enemies
        .into_par_iter()
        .map(|enemy| move_enemy_for_type(player, enemy, walls, delta))
        .collect()
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;

    use crate::{
        constants::{ENEMY_SIZE, PLAYER_SIZE},
        model::TextureId,
    };

    use super::*;

    #[test]
    fn test_move_enemy_for_type() {
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
            texture: TextureId::Debug,
            start: vec2(-5.0, 0.0),
            end: vec2(5.0, 0.0),
        }];

        let delta = 1.0;

        let moved_enemy = move_enemy_for_type(&player, enemy.clone(), &walls, delta);

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

        let not_moved_enemy = move_enemy_for_type(&player, far_enemy.clone(), &walls, delta);
        assert_eq!(not_moved_enemy.entity.position, far_enemy.entity.position);
    }
}
