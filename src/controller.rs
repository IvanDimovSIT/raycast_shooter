use macroquad::math::{vec2, Vec2};

use crate::{constants::{MOVE_SPEED, TURN_SPEED}, input::Operation, math::rotate_point, model::{Entity, Player, Wall}, service::move_entity};

pub struct GameObjects<'a>{
    pub player: &'a Player,
    pub walls: &'a [Wall],
}


fn handle_left(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), TURN_SPEED*delta);
    Player { entity: player.entity, look }  
}


fn handle_right(player: Player, delta: f32) -> Player {
    let look = rotate_point(player.look, vec2(0.0, 0.0), -TURN_SPEED*delta);
    Player { entity: player.entity, look }  
}

fn handle_forward(game_objects: &GameObjects, delta: f32) -> Player {
    Player {
        entity: move_entity(game_objects.player.entity, game_objects.player.look*delta*MOVE_SPEED, game_objects.walls),
        look: game_objects.player.look
    }
}


fn handle_back(game_objects: &GameObjects, delta: f32) -> Player {
    Player {
        entity: move_entity(game_objects.player.entity, -game_objects.player.look*delta*MOVE_SPEED, game_objects.walls),
        look: game_objects.player.look
    }
}


pub fn handle_input(game_objects: GameObjects, operations: &[Operation], delta: f32) -> Player {
    operations.into_iter()
        .fold(*game_objects.player,|pl, op| match op {
            Operation::Left => handle_left(pl, delta),
            Operation::Right => handle_right(pl, delta),
            Operation::Forward => handle_forward(&game_objects, delta),
            Operation::Back => handle_back(&game_objects, delta),
        })
}