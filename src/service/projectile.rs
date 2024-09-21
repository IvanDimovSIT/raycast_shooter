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
    if check_circles_collide(
        projectile.entity.position,
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
    } else if let Some(hit) =
        find_projectile_hit_walls(projectile.entity.position, new_position, walls)
    {
        (
            None,
            vec![GameEvent::LocationShot {
                position: hit
                    - projectile.direction * CREATE_GUNSHOT_HIT_ANIMATION_OFFSET_TO_CAMERA,
            }],
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
