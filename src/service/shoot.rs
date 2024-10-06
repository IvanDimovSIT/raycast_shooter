use macroquad::math::Vec2;

use crate::{
    constants::{
        CORPSE_OFFSET, CORPSE_SIZE, CREATE_GUNSHOT_HIT_ANIMATION_OFFSET_TO_CAMERA,
        GUNSHOT_ANIMATION_LENGTH, GUNSHOT_ANIMATION_SPEED, GUN_DAMAGE, MAX_BULLETS,
        MAX_SHOOT_DISTANCE, RELOAD_SPEED, SHOOT_SPEED,
    },
    math::{find_intersection, line_intersects_circle},
    model::{
        decoration::{Decoration, DecorationGraphics},
        enemy::Enemy,
        Animation, Entity, GameEvent, Player, PlayerInfo, ShootingStatus, TextureId, Wall,
    },
};

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
        graphics: DecorationGraphics::Texture(TextureId::Skull),
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

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;

    use crate::constants::{ENEMY_SIZE, PLAYER_SIZE};

    use super::*;

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
