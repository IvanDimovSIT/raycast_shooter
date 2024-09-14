use macroquad::math::{vec2, Vec2};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    constants::{
        CORPSE_OFFSET, CORPSE_SIZE, CREATE_GUNSHOT_HIT_ANIMATION_OFFSET_TO_CAMERA,
        ENEMY_ATTACK_RANGE, ENEMY_DPS, ENEMY_MAX_CHASE_DISTANCE, ENEMY_MOVE_SPEED,
        GUNSHOT_ANIMATION_LENGTH, GUNSHOT_ANIMATION_SPEED, GUN_DAMAGE, MAX_BULLETS,
        MAX_SHOOT_DISTANCE, MOVE_SPEED, RELOAD_SPEED, SHOOT_SPEED,
    },
    math::{check_circles_collide, find_intersection, line_intersects_circle, rotate_point},
    model::{
        decoration::{Decoration, DecorationGraphics},
        enemy::Enemy,
        key_object::KeyObject,
        Animation, Entity, GameEvent, GameObjects, Player, PlayerInfo, ShootingStatus, Texture,
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

fn update_shoot_shooting(time_since_last_shot: f32, player_info: PlayerInfo) -> (PlayerInfo, bool) {
    if player_info.bullets == 0 {
        return (
            PlayerInfo {
                shooting_status: ShootingStatus::Reloading,
                time_since_last_shot,
                ..player_info
            },
            false,
        );
    }

    if time_since_last_shot < SHOOT_SPEED {
        return (
            PlayerInfo {
                time_since_last_shot,
                ..player_info
            },
            false,
        );
    }

    if player_info.bullets == 1 {
        (
            PlayerInfo {
                shooting_status: ShootingStatus::Reloading,
                time_since_last_shot: 0.0,
                bullets: 0,
                ..player_info
            },
            true,
        )
    } else {
        (
            PlayerInfo {
                time_since_last_shot: 0.0,
                bullets: player_info.bullets - 1,
                ..player_info
            },
            true,
        )
    }
}

fn update_shoot_not_shooting(
    time_since_last_shot: f32,
    player_info: PlayerInfo,
) -> (PlayerInfo, bool) {
    if time_since_last_shot >= RELOAD_SPEED {
        (
            PlayerInfo {
                time_since_last_shot,
                bullets: MAX_BULLETS,
                ..player_info
            },
            false,
        )
    } else {
        (
            PlayerInfo {
                time_since_last_shot,
                ..player_info
            },
            false,
        )
    }
}

fn update_shoot_reloading(
    time_since_last_shot: f32,
    player_info: PlayerInfo,
) -> (PlayerInfo, bool) {
    if time_since_last_shot >= RELOAD_SPEED {
        (
            PlayerInfo {
                shooting_status: ShootingStatus::NotShooting,
                time_since_last_shot,
                bullets: MAX_BULLETS,
                ..player_info
            },
            false,
        )
    } else {
        (
            PlayerInfo {
                time_since_last_shot,
                ..player_info
            },
            false,
        )
    }
}

pub fn update_shoot(player_info: PlayerInfo, delta: f32) -> (PlayerInfo, bool) {
    let time_since_last_shot = player_info.time_since_last_shot + delta;

    match player_info.shooting_status {
        ShootingStatus::Shooting => update_shoot_shooting(time_since_last_shot, player_info),
        ShootingStatus::NotShooting => update_shoot_not_shooting(time_since_last_shot, player_info),
        ShootingStatus::Reloading => update_shoot_reloading(time_since_last_shot, player_info),
    }
}

fn find_shot_enemy<'a>(
    player: &'a Player,
    enemies: &'a [Enemy],
    walls: &'a [Wall],
) -> (Option<&'a Enemy>, Option<Vec2>) {
    let shoot_ray = player.entity.position + player.look.normalize_or_zero() * MAX_SHOOT_DISTANCE;

    let closest_hit_wall = walls
        .iter()
        .filter_map(|wall| {
            find_intersection(player.entity.position, shoot_ray, wall.start, wall.end)
        })
        .map(|p| (p.distance(player.entity.position), p))
        .min_by(|a, b| a.0.total_cmp(&b.0));

    let max_distance = if let Some(some) = closest_hit_wall {
        some.0
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

    if let Some(some) = hit_enemy {
        (Some(some.0), Some(some.0.entity.position))
    } else if let Some(some) = closest_hit_wall {
        (None, Some(some.1))
    } else {
        (None, None)
    }
}

fn create_shot_particles_event(shot_location: Vec2) -> GameEvent {
    GameEvent::LocationShot {
        position: shot_location,
    }
}

pub fn shoot_enemies(
    player: &Player,
    mut enemies: Vec<Enemy>,
    walls: &[Wall],
) -> (Vec<Enemy>, Vec<GameEvent>) {
    let (shot_enemy_option, shot_location) = find_shot_enemy(player, &enemies, walls);

    let shot_event = if let Some(some) = shot_location {
        vec![create_shot_particles_event(some)]
    } else {
        vec![]
    };

    if shot_enemy_option.is_none() {
        return (enemies, shot_event);
    }
    let shot_enemy_id = shot_enemy_option.unwrap().id;

    enemies
        .iter_mut()
        .find(|enemy| enemy.id == shot_enemy_id)
        .map(|enemy| {
            *enemy = Enemy {
                hp: enemy.hp - GUN_DAMAGE,
                ..*enemy
            };
        });

    let game_events = enemies
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
        .chain(shot_event)
        .collect();

    enemies.retain(|enemy| enemy.hp > 0.0);

    (enemies, game_events)
}

pub fn create_corpse(location: Vec2) -> Decoration {
    Decoration {
        entity: Entity {
            position: location,
            size: CORPSE_SIZE,
        },
        graphics: DecorationGraphics::Texture(Texture::Skull),
        life: None,
        offset: CORPSE_OFFSET,
    }
}

pub fn start_shooting(player_info: PlayerInfo) -> PlayerInfo {
    if matches!(player_info.shooting_status, ShootingStatus::NotShooting) {
        PlayerInfo {
            shooting_status: ShootingStatus::Shooting,
            ..player_info
        }
    } else {
        player_info
    }
}

pub fn stop_shooting(player_info: PlayerInfo) -> PlayerInfo {
    if matches!(player_info.shooting_status, ShootingStatus::Shooting) {
        PlayerInfo {
            shooting_status: ShootingStatus::NotShooting,
            ..player_info
        }
    } else {
        player_info
    }
}

pub fn turn_player(player: Player, thetha: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), thetha);
    Player { look, ..player }
}

pub fn create_shot_animation_decoration(player: &Player, location: Vec2) -> Decoration {
    let dir_to_player = (player.entity.position - location).normalize_or_zero();
    let offset_to_camera = dir_to_player * CREATE_GUNSHOT_HIT_ANIMATION_OFFSET_TO_CAMERA;
    let position = location + offset_to_camera;

    Decoration {
        entity: Entity {
            position,
            size: 0.2,
        },
        graphics: DecorationGraphics::Animation {
            animation: Animation::Explosion,
            animation_speed: GUNSHOT_ANIMATION_SPEED,
        },
        life: Some(GUNSHOT_ANIMATION_LENGTH),
        offset: 0.1,
    }
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
    game_objects.exit_triggers
        .iter()
        .any(|trigger| 
            check_circles_collide(
                trigger.position,
                trigger.size,
                game_objects.player.entity.position,
                game_objects.player.entity.size
            )
        )
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

    #[test]
    fn test_shoot_enemies() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: PLAYER_SIZE,
            },
            look: vec2(1.0, 0.0),
        };

        let enemy = Enemy {
            entity: Entity {
                position: vec2(MAX_SHOOT_DISTANCE - 1.0, 0.0),
                size: ENEMY_SIZE,
            },
            hp: 100.0,
            ..Default::default()
        };

        let walls = vec![];

        let (remaining_enemies, game_events) = shoot_enemies(&player, vec![enemy.clone()], &walls);

        assert_eq!(remaining_enemies[0].hp, 100.0 - GUN_DAMAGE);

        assert!(matches!(game_events[0], GameEvent::LocationShot { .. }));
    }

    #[test]
    fn test_update_shoot_shooting() {
        let player_info = PlayerInfo {
            bullets: 5,
            shooting_status: ShootingStatus::Shooting,
            time_since_last_shot: SHOOT_SPEED,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) = update_shoot(player_info.clone(), 0.0);

        assert_eq!(updated_player_info.bullets, 4);
        assert!(shot_fired);

        let player_info_not_ready = PlayerInfo {
            bullets: 5,
            shooting_status: ShootingStatus::Shooting,
            time_since_last_shot: SHOOT_SPEED - 0.1,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) = update_shoot(player_info_not_ready, 0.0);

        assert_eq!(updated_player_info.bullets, 5);
        assert!(!shot_fired);

        let player_info_one_bullet = PlayerInfo {
            bullets: 1,
            shooting_status: ShootingStatus::Shooting,
            time_since_last_shot: SHOOT_SPEED,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) = update_shoot(player_info_one_bullet.clone(), 0.0);

        assert_eq!(updated_player_info.bullets, 0);
        assert_eq!(
            updated_player_info.shooting_status,
            ShootingStatus::Reloading
        );
        assert!(shot_fired);

        let player_info_no_bullets = PlayerInfo {
            bullets: 0,
            shooting_status: ShootingStatus::Shooting,
            time_since_last_shot: SHOOT_SPEED,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) = update_shoot(player_info_no_bullets.clone(), 0.0);

        assert_eq!(updated_player_info.bullets, 0);
        assert_eq!(
            updated_player_info.shooting_status,
            ShootingStatus::Reloading
        );
        assert!(!shot_fired);
    }

    #[test]
    fn test_update_shoot_not_shooting() {
        let player_info = PlayerInfo {
            bullets: 5,
            shooting_status: ShootingStatus::NotShooting,
            time_since_last_shot: RELOAD_SPEED * 0.5,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) =
            update_shoot(player_info.clone(), RELOAD_SPEED * 0.4);

        assert_eq!(updated_player_info.bullets, 5);
        assert!(!shot_fired);

        let player_info_needs_reload = PlayerInfo {
            bullets: 0,
            shooting_status: ShootingStatus::NotShooting,
            time_since_last_shot: RELOAD_SPEED * 0.95,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) =
            update_shoot(player_info_needs_reload.clone(), RELOAD_SPEED * 0.1);

        assert_eq!(updated_player_info.bullets, MAX_BULLETS);
        assert!(!shot_fired);
    }

    #[test]
    fn test_update_shoot_reloading() {
        let player_info = PlayerInfo {
            bullets: 0,
            shooting_status: ShootingStatus::Reloading,
            time_since_last_shot: RELOAD_SPEED - 0.5,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) = update_shoot(player_info.clone(), 0.4);

        assert_eq!(
            updated_player_info.shooting_status,
            ShootingStatus::Reloading
        );
        assert_eq!(updated_player_info.bullets, 0);
        assert!(!shot_fired);

        let player_info_reloaded = PlayerInfo {
            bullets: 0,
            shooting_status: ShootingStatus::Reloading,
            time_since_last_shot: RELOAD_SPEED - 0.5,
            ..Default::default()
        };

        let (updated_player_info, shot_fired) = update_shoot(player_info_reloaded.clone(), 0.6);

        assert_eq!(updated_player_info.bullets, MAX_BULLETS);
        assert_eq!(
            updated_player_info.shooting_status,
            ShootingStatus::NotShooting
        );
        assert!(!shot_fired);
    }
}
