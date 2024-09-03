use std::mem::take;

use macroquad::{
    math::{vec2, Vec2},
    rand::{gen_range, rand},
};
use rayon::iter;
use uuid::Uuid;

use crate::{
    constants::{CREATE_GUNSHOT_ANIMATION_RATE, MOVE_SPEED, TURN_SPEED},
    input::Operation,
    math::{find_perpendicular_vector, rotate_point},
    model::{decoration::Decoration, GameEvent, GameObjects, Player, PlayerInfo},
    service::{
        check_pickup_key, create_corpse, create_shot_animation_decoration,
        move_enemies_towards_player, move_player, shoot_enemies,
    },
};

fn handle_left(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), TURN_SPEED * delta);
    Player { look, ..player }
}

fn handle_right(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), -TURN_SPEED * delta);
    Player { look, ..player }
}

fn handle_forward(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            player.look * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        ..player
    }
}

fn handle_back(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            -player.look * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        ..player
    }
}

fn handle_strafe_left(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            find_perpendicular_vector(player.look) * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        ..player
    }
}

fn handle_strafe_right(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            -find_perpendicular_vector(player.look) * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        ..player
    }
}

fn handle_start_shooting(player_info: &PlayerInfo) -> PlayerInfo {
    PlayerInfo {
        is_shooting: true,
        ..player_info.clone()
    }
}

fn handle_stop_shooting(player_info: &PlayerInfo) -> PlayerInfo {
    PlayerInfo {
        is_shooting: false,
        ..player_info.clone()
    }
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
            Operation::Left => (handle_left(pl, delta), info),
            Operation::Right => (handle_right(pl, delta), info),
            Operation::Forward => (handle_forward(game_objects, pl, delta), info),
            Operation::Back => (handle_back(game_objects, pl, delta), info),
            Operation::StrafeLeft => (handle_strafe_left(game_objects, pl, delta), info),
            Operation::StrafeRight => (handle_strafe_right(game_objects, pl, delta), info),
            Operation::StartShooting => (pl, handle_start_shooting(&info)),
            Operation::StopShooting => (pl, handle_stop_shooting(&info)),
        },
    )
}

fn handle_pickup_key(game_objects: &mut GameObjects, key_id: &Uuid) {
    game_objects.keys.retain(|key| key.id != *key_id);
    println!("Picked up key:{}", key_id);
}

fn handle_enemy_killed(game_objects: &mut GameObjects, position: Vec2) {
    game_objects.decorations = take(&mut game_objects.decorations)
        .into_iter()
        .chain(std::iter::once(create_corpse(position)))
        .collect();

    println!("Enemy killed at:{}", position);
}

fn handle_location_shot(game_objects: &mut GameObjects, position: Vec2, delta: f32) {
    if delta * CREATE_GUNSHOT_ANIMATION_RATE > gen_range(0.0, 1.0) {
        game_objects.decorations = take(&mut game_objects.decorations)
            .into_iter()
            .chain(std::iter::once(create_shot_animation_decoration(
                &game_objects.player,
                position,
            )))
            .collect();
    }
}

fn handle_events(game_objects: &mut GameObjects, events: &[GameEvent], delta: f32) {
    for e in events {
        match e {
            GameEvent::PickUpKey(key_id) => handle_pickup_key(game_objects, key_id),
            GameEvent::EnemyKilled { position } => handle_enemy_killed(game_objects, *position),
            GameEvent::LocationShot { position } => {
                handle_location_shot(game_objects, *position, delta)
            }
        }
    }
}

fn update_decorations(decorations: Vec<Decoration>, delta: f32) -> Vec<Decoration> {
    decorations
        .into_iter()
        .filter_map(|decor| decor.update(delta))
        .collect()
}

pub fn next_game_step(mut game_objects: GameObjects, delta: f32) -> GameObjects {
    game_objects.enemies = move_enemies_towards_player(
        &game_objects.player,
        game_objects.enemies,
        &game_objects.walls,
        delta,
    );
    let kill_enemies_events;
    (game_objects.enemies, kill_enemies_events) = shoot_enemies(
        &game_objects.player,
        &game_objects.player_info,
        game_objects.enemies,
        &game_objects.walls,
        delta,
    );
    game_objects.decorations = update_decorations(game_objects.decorations, delta);

    let events: Vec<_> = check_pickup_key(&game_objects.player, &game_objects.keys)
        .into_iter()
        .chain(kill_enemies_events)
        .collect();

    handle_events(&mut game_objects, &events, delta);

    game_objects
}
