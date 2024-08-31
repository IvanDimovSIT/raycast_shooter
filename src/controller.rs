use macroquad::math::{vec2, Vec2};
use uuid::Uuid;

use crate::{
    constants::{MOVE_SPEED, TURN_SPEED},
    input::Operation,
    math::{find_perpendicular_vector, rotate_point},
    model::{GameEvent, GameObjects, Player, PlayerInfo},
    service::{check_pickup_key, shoot_enemies, move_enemies_towards_player, move_player},
};

fn handle_left(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), TURN_SPEED * delta);
    Player {
        look,
        ..player
    }
}

fn handle_right(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), -TURN_SPEED * delta);
    Player {
        look,
        ..player
    }
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
        (game_objects.player, game_objects.player_info.clone()),
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

fn handle_enemy_killed(poition: Vec2) {
    println!("Enemy killed at:{}", poition);
}

fn handle_events(game_objects: &mut GameObjects, events: &[GameEvent]) {
    for e in events {
        match e {
            GameEvent::PickUpKey(key_id) => handle_pickup_key(game_objects, key_id),
            GameEvent::EnemyKilled { position } => handle_enemy_killed(*position),
        }
    }
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

    let events: Vec<_> = check_pickup_key(&game_objects.player, &game_objects.keys)
        .into_iter()
        .chain(kill_enemies_events)
        .collect();

    handle_events(&mut game_objects, &events);

    game_objects
}
