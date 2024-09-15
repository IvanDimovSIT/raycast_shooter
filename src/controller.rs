use std::mem::take;

use macroquad::math::Vec2;
use uuid::Uuid;

use crate::{
    input::Operation,
    math::find_perpendicular_vector,
    model::{decoration::Decoration, GameEvent, GameObjects, Player, PlayerInfo},
    service::enemy::*,
    service::player::*,
    service::shoot::*,
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

fn handle_pickup_key(game_objects: &mut GameObjects, key_id: &Uuid) {
    game_objects.keys.retain(|key| key.id != *key_id);
    game_objects.player_info = PlayerInfo {
        picked_up_keys: game_objects.player_info.picked_up_keys + 1,
        ..game_objects.player_info
    };
    println!("Picked up key:{}", key_id);
}

fn handle_enemy_killed(game_objects: &mut GameObjects, position: Vec2) {
    game_objects.decorations = take(&mut game_objects.decorations)
        .into_iter()
        .chain(std::iter::once(create_corpse(position)))
        .collect();

    println!("Enemy killed at:{}", position);
}

fn handle_location_shot(game_objects: &mut GameObjects, position: Vec2) {
    game_objects.decorations = take(&mut game_objects.decorations)
        .into_iter()
        .chain(std::iter::once(create_shot_animation_decoration(
            &game_objects.player,
            position,
        )))
        .collect();
}

fn handle_player_take_damage(game_objects: &mut GameObjects, damage: f32) {
    game_objects.player_info.health -= damage;
    println!("Player took damage({damage})");
}

fn handle_create_projectile(game_objects: &mut GameObjects, position: Vec2, direction: Vec2) {
    todo!(
        "handle_create_projectile {:?}{:?}{:?}",
        game_objects,
        position,
        direction
    )
}

fn handle_events(game_objects: &mut GameObjects, events: &[GameEvent]) {
    for e in events {
        match e {
            GameEvent::PickUpKey(key_id) => handle_pickup_key(game_objects, key_id),
            GameEvent::EnemyKilled { position } => handle_enemy_killed(game_objects, *position),
            GameEvent::LocationShot { position } => handle_location_shot(game_objects, *position),
            GameEvent::PlayerTakeDamage(damage) => handle_player_take_damage(game_objects, *damage),
            GameEvent::CreateProjectile {
                position,
                direction,
            } => handle_create_projectile(game_objects, *position, *direction),
        }
    }
}

fn update_decorations(decorations: Vec<Decoration>, delta: f32) -> Vec<Decoration> {
    decorations
        .into_iter()
        .filter_map(|decor| decor.update(delta))
        .collect()
}

pub fn next_game_step(game_objects: &mut GameObjects, delta: f32) {
    game_objects.enemies = move_enemies_towards_player(
        &game_objects.player,
        take(&mut game_objects.enemies),
        &game_objects.walls,
        delta,
    );

    let can_shoot;
    (game_objects.player_info, can_shoot) =
        update_shoot(take(&mut game_objects.player_info), delta);

    let kill_enemies_events;
    (game_objects.enemies, kill_enemies_events) = if can_shoot {
        shoot_enemies(
            &game_objects.player,
            take(&mut game_objects.enemies),
            &game_objects.walls,
        )
    } else {
        (take(&mut game_objects.enemies), vec![])
    };

    let attack_events;
    (game_objects.enemies, attack_events) = enemies_attack_player(
        &game_objects.player,
        take(&mut game_objects.enemies),
        &game_objects.walls,
        delta,
    );

    game_objects.decorations = update_decorations(take(&mut game_objects.decorations), delta);

    let events: Vec<_> = check_pickup_key(&game_objects.player, &game_objects.keys)
        .into_iter()
        .chain(kill_enemies_events)
        .chain(attack_events)
        .collect();

    handle_events(game_objects, &events);
}

pub fn reset_state(mut game_objects: GameObjects) -> GameObjects {
    game_objects.player_info = stop_shooting(game_objects.player_info);

    game_objects
}

pub fn is_game_over(game_objects: &GameObjects) -> bool {
    game_objects.player_info.health <= 0.0
}

pub fn is_game_won(game_objects: &GameObjects) -> bool {
    game_objects.keys.len() == 0 && is_player_at_exit(game_objects)
}
