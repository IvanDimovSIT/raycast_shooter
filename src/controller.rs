use std::mem::take;

use macroquad::math::Vec2;

use crate::{
    constants::RANGED_ENEMY_SHOT_SIZE,
    input::Operation,
    math::find_perpendicular_vector,
    model::{
        decoration::Decoration, projectile::Projectile, Entity, GameEvent, GameObjects, Player,
        PlayerInfo, SoundId, Texture,
    },
    service::{
        enemy::*, key::check_pickup_key, player::*, projectile::update_projctiles, shoot::*,
    },
    sound_manager::SoundManager,
};

fn handle_left(player: Player, angle: f32, delta: f32) -> Player {
    turn_player(player, angle * delta)
}

fn handle_right(player: Player, angle: f32, delta: f32) -> Player {
    turn_player(player, -angle * delta)
}

fn handle_forward(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    let direction = player.look;
    move_player(game_objects, player, direction, delta)
}

fn handle_back(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    let direction = -player.look;
    move_player(game_objects, player, direction, delta)
}

fn handle_strafe_left(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    let direction = find_perpendicular_vector(player.look);
    move_player(game_objects, player, direction, delta)
}

fn handle_strafe_right(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    let direction = -find_perpendicular_vector(player.look);
    move_player(game_objects, player, direction, delta)
}

fn handle_shoot(player_info: &PlayerInfo) -> PlayerInfo {
    start_shooting(player_info.clone())
}

pub fn handle_input(
    game_objects: &GameObjects,
    operations: &[Operation],
    delta: f32,
) -> (Player, PlayerInfo) {
    operations.iter().fold(
        (
            game_objects.player.clone(),
            game_objects.player_info.clone(),
        ),
        |(pl, info), op| match op {
            Operation::Left(angle) => (handle_left(pl, *angle, delta), info),
            Operation::Right(angle) => (handle_right(pl, *angle, delta), info),
            Operation::Forward => (handle_forward(game_objects, pl, delta), info),
            Operation::Back => (handle_back(game_objects, pl, delta), info),
            Operation::StrafeLeft => (handle_strafe_left(game_objects, pl, delta), info),
            Operation::StrafeRight => (handle_strafe_right(game_objects, pl, delta), info),
            Operation::Shoot => (pl, handle_shoot(&info)),
        },
    )
}

fn handle_pickup_key(sound_manager: &SoundManager, game_objects: &mut GameObjects) {
    game_objects.player_info = PlayerInfo {
        picked_up_keys: game_objects.player_info.picked_up_keys + 1,
        ..game_objects.player_info
    };
    sound_manager.play(SoundId::PickUpKey);
    println!("Picked up key");
}

fn handle_enemy_killed(game_objects: &mut GameObjects, position: Vec2) {
    game_objects.decorations = take(&mut game_objects.decorations)
        .into_iter()
        .chain(std::iter::once(create_corpse(position)))
        .collect();

    println!("Enemy killed at:{}", position);
}

fn handle_location_shot(
    sound_manager: &SoundManager,
    game_objects: &mut GameObjects,
    position: Vec2,
) {
    game_objects.decorations = take(&mut game_objects.decorations)
        .into_iter()
        .chain(std::iter::once(create_shot_animation_decoration(
            &game_objects.player,
            position,
        )))
        .collect();

    sound_manager.play(SoundId::ShotHit);
}

fn handle_player_take_damage(
    sound_manager: &SoundManager,
    game_objects: &mut GameObjects,
    damage: f32,
) {
    game_objects.player_info.health -= damage;
    sound_manager.play(SoundId::PlayerTakeDamage);
    println!("Player took damage({damage})");
}

fn handle_create_projectile(
    game_objects: &mut GameObjects,
    position: Vec2,
    direction: Vec2,
    damage: f32,
) {
    game_objects.projectiles.push(Projectile {
        entity: Entity {
            position,
            size: RANGED_ENEMY_SHOT_SIZE,
        },
        direction,
        damage,
        texture: Texture::Projectile,
    });
}

pub fn handle_events(
    sound_manager: &SoundManager,
    game_objects: &mut GameObjects,
    events: &[GameEvent],
) {
    for e in events {
        match e {
            GameEvent::PickUpKey => handle_pickup_key(sound_manager, game_objects),
            GameEvent::EnemyKilled { position } => handle_enemy_killed(game_objects, *position),
            GameEvent::LocationShot { position } => {
                handle_location_shot(sound_manager, game_objects, *position)
            }
            GameEvent::PlayerTakeDamage(damage) => {
                handle_player_take_damage(sound_manager, game_objects, *damage)
            }
            GameEvent::CreateProjectile {
                position,
                direction,
                damage,
            } => handle_create_projectile(game_objects, *position, *direction, *damage),
        }
    }
}

fn update_decorations(decorations: Vec<Decoration>, delta: f32) -> Vec<Decoration> {
    decorations
        .into_iter()
        .filter_map(|decor| decor.update(delta))
        .collect()
}

pub fn next_game_step(game_objects: GameObjects, delta: f32) -> (GameObjects, Vec<GameEvent>) {
    let moved_enemies = move_enemies_towards_player(
        &game_objects.player,
        game_objects.enemies,
        &game_objects.walls,
        delta,
    );

    let (player_info_shoot, can_shoot) = update_shoot(game_objects.player_info, delta);

    let (shot_enemies, kill_enemies_events) = if can_shoot {
        shoot_enemies(&game_objects.player, moved_enemies, &game_objects.walls)
    } else {
        (moved_enemies, vec![])
    };

    let (attacked_enemies, attack_events) = enemies_attack_player(
        &game_objects.player,
        shot_enemies,
        &game_objects.walls,
        delta,
    );

    let updated_decorations = update_decorations(game_objects.decorations, delta);

    let (projectiles, projectile_events) = update_projctiles(
        game_objects.projectiles,
        &game_objects.player,
        &game_objects.walls,
        delta,
    );

    let new_player_info = PlayerInfo {
        health: regenerate_health(player_info_shoot.health, delta),
        ..player_info_shoot
    };

    let (new_keys, key_events) = check_pickup_key(&game_objects.player, game_objects.keys);
    let events: Vec<_> = key_events
        .into_iter()
        .chain(kill_enemies_events)
        .chain(attack_events)
        .chain(projectile_events)
        .collect();

    let new_game_objects = GameObjects {
        player: game_objects.player,
        player_info: new_player_info,
        walls: game_objects.walls,
        enemies: attacked_enemies,
        keys: new_keys,
        exit_triggers: game_objects.exit_triggers,
        decorations: updated_decorations,
        projectiles,
    };

    (new_game_objects, events)
}

pub fn play_sounds(sound_manager: &mut SoundManager, game_objects: &GameObjects) {
    if matches!(
        game_objects.player_info.shooting_status,
        crate::model::ShootingStatus::Shooting
    ) {
        sound_manager.start_looped(SoundId::Shooting);
    } else {
        sound_manager.stop_looped(SoundId::Shooting);
    }
}

pub fn reset_state(game_objects: &mut GameObjects) {
    game_objects.player_info = stop_shooting(game_objects.player_info.clone());
}

pub fn is_game_over(game_objects: &GameObjects) -> bool {
    game_objects.player_info.health <= 0.0
}

pub fn is_game_won(game_objects: &GameObjects) -> bool {
    game_objects.keys.is_empty() && is_player_at_exit(game_objects)
}
