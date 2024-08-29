use macroquad::math::vec2;

use crate::{
    constants::{MOVE_SPEED, TURN_SPEED},
    input::Operation,
    math::rotate_point,
    model::{Player, Wall},
    service::move_entity,
};

pub struct GameObjects<'a> {
    pub player: &'a Player,
    pub walls: &'a [Wall],
}

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
        entity: move_entity(
            player.entity,
            player.look * delta * MOVE_SPEED,
            game_objects.walls,
        ),
        look: player.look,
    }
}

fn handle_back(game_objects: &GameObjects, player: Player, delta: f32) -> Player {
    Player {
        entity: move_entity(
            player.entity,
            -player.look * delta * MOVE_SPEED,
            game_objects.walls,
        ),
        look: player.look,
    }
}

pub fn handle_input(game_objects: GameObjects, operations: &[Operation], delta: f32) -> Player {
    operations
        .iter()
        .fold(*game_objects.player, |pl, op| match op {
            Operation::Left => handle_left(pl, delta),
            Operation::Right => handle_right(pl, delta),
            Operation::Forward => handle_forward(&game_objects, pl, delta),
            Operation::Back => handle_back(&game_objects, pl, delta),
        })
}
