use macroquad::math::Vec2;

use crate::{
    constants::CREATE_GUNSHOT_HIT_ANIMATION_OFFSET_TO_CAMERA,
    math::{check_circles_collide, find_intersection},
    model::{projectile::Projectile, Entity, GameEvent, Player, Wall},
};

fn find_projectile_hit_walls(
    old_position: Vec2,
    new_position: Vec2,
    walls: &[Wall],
) -> Option<Vec2> {
    walls
        .iter()
        .filter_map(|wall| find_intersection(wall.start, wall.end, old_position, new_position))
        .next()
}

fn update_projectile(
    projectile: &Projectile,
    player: &Player,
    walls: &[Wall],
    delta: f32,
) -> (Option<Projectile>, Vec<GameEvent>) {
    let new_position = projectile.entity.position + projectile.direction * delta;
    if let Some(hit) = find_projectile_hit_walls(projectile.entity.position, new_position, walls) {
        (
            None,
            vec![GameEvent::LocationShot {
                position: hit
                    - projectile.direction * CREATE_GUNSHOT_HIT_ANIMATION_OFFSET_TO_CAMERA,
            }],
        )
    } else if check_circles_collide(
        new_position,
        projectile.entity.size,
        player.entity.position,
        player.entity.size,
    ) {
        (
            None,
            vec![
                GameEvent::PlayerTakeDamage(projectile.damage),
                GameEvent::LocationShot {
                    position: new_position,
                },
            ],
        )
    } else {
        (
            Some(Projectile {
                entity: Entity {
                    position: new_position,
                    ..projectile.entity
                },
                ..*projectile
            }),
            vec![],
        )
    }
}

pub fn update_projctiles(
    projectiles: Vec<Projectile>,
    player: &Player,
    walls: &[Wall],
    delta: f32,
) -> (Vec<Projectile>, Vec<GameEvent>) {
    let (new_projectiles, events): (Vec<_>, Vec<_>) = projectiles
        .into_iter()
        .map(|projectile| update_projectile(&projectile, player, walls, delta))
        .unzip();

    (
        new_projectiles.into_iter().flatten().collect(),
        events.into_iter().flatten().collect(),
    )
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;

    use crate::model::Texture;

    use super::*;

    #[test]
    fn test_update_projectile_hit_player() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 5.0),
                size: 1.0,
            },
            look: vec2(0.0, 0.0),
        };

        let walls = vec![];

        let delta = 1.0;

        let projectile = Projectile {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: 0.5,
            },
            direction: vec2(0.0, 5.0),
            damage: 10.0,
            texture: Texture::default(),
        };

        let (new_projectile, events) = update_projectile(&projectile, &player, &walls, delta);
        dbg!(&new_projectile);
        dbg!(&events);
        assert!(new_projectile.is_none());
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], GameEvent::PlayerTakeDamage(damage) if damage == 10.0));
    }

    #[test]
    fn test_update_projectile_miss_player_and_walls() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 100.0),
                size: 1.0,
            },
            look: vec2(0.0, 0.0),
        };

        let walls = vec![];

        let delta = 1.0;

        let projectile = Projectile {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: 0.5,
            },
            direction: vec2(1.0, 0.0),
            damage: 1.0,
            texture: Texture::default(),
        };

        let (new_projectile, events) = update_projectile(&projectile, &player, &walls, delta);

        assert!(new_projectile.is_some());
        assert_eq!(events.len(), 0);
        assert_eq!(new_projectile.unwrap().entity.position, vec2(1.0, 0.0));
    }

    #[test]
    fn test_update_projectile_hit_wall() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 10.0),
                size: 1.0,
            },
            look: vec2(0.0, 0.0),
        };

        let walls = vec![Wall {
            texture: Texture::default(),
            start: vec2(-10.0, 5.0),
            end: vec2(10.0, 5.0),
        }];

        let delta = 1.0;

        let projectile = Projectile {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: 0.5,
            },
            direction: vec2(0.0, 10.0),
            damage: 1.0,
            texture: Texture::default(),
        };

        let (new_projectile, events) = update_projectile(&projectile, &player, &walls, delta);
        assert_eq!(events.len(), 1);
        assert!(new_projectile.is_none());
    }
}
