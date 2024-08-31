use macroquad::math::vec2;
use uuid::Uuid;

use crate::{
    constants::{MOVE_SPEED, TURN_SPEED},
    input::Operation,
    math::{find_perpendicular_vector, rotate_point},
    model::{GameEvent, GameObjects, Player, PlayerInfo},
    service::move_player,
};

fn handle_left(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), TURN_SPEED * delta);
    Player {
        entity: player.entity,
        look,
    }
}

fn handle_right(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), -TURN_SPEED * delta);
    Player {
        entity: player.entity,
        look,
    }
}

fn handle_forward(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            player.look * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        look: player.look,
    }
}

fn handle_back(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            -player.look * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        look: player.look,
    }
}

fn handle_strafe_left(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            find_perpendicular_vector(player.look) * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        look: player.look,
    }
}

fn handle_strafe_right(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_player(
            player.entity,
            -find_perpendicular_vector(player.look) * delta * MOVE_SPEED,
            &game_objects.walls,
        ),
        look: player.look,
    }
}

fn handle_start_shooting(player_info: &PlayerInfo) -> PlayerInfo {
    PlayerInfo {
        is_shooting: true,
        health: player_info.health,
    }
}

fn handle_stop_shooting(player_info: &PlayerInfo) -> PlayerInfo {
    PlayerInfo {
        is_shooting: false,
        health: player_info.health,
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
            Operation::Forward => (handle_forward(&game_objects, pl, delta), info),
            Operation::Back => (handle_back(&game_objects, pl, delta), info),
            Operation::StrafeLeft => (handle_strafe_left(&game_objects, pl, delta), info),
            Operation::StrafeRight => (handle_strafe_right(&game_objects, pl, delta), info),
            Operation::StartShooting => (pl, handle_start_shooting(&info)),
            Operation::StopShooting => (pl, handle_stop_shooting(&info)),
        },
    )
}

fn handle_pickup_key(game_objects: &mut GameObjects, key_id: &Uuid) {
    game_objects.keys.retain(|key| key.id != *key_id);
    println!("Picked up key:{}", key_id);
}

pub fn handle_events(game_objects: &mut GameObjects, events: &[GameEvent]) {
    for e in events {
        match e {
            GameEvent::PickUpKey(key_id) => handle_pickup_key(game_objects, key_id),
        }
    }
}
