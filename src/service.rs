use macroquad::math::Vec2;

use crate::{
    math::line_intersects_circle,
    model::{Entity, Wall},
};

pub fn move_entity(entity: Entity, movement: Vec2, walls: &[Wall]) -> Entity {
    let new_pos = entity.position + movement;

    let is_collision = walls
        .iter()
        .any(|wall| line_intersects_circle(wall.start, wall.end, new_pos, entity.size));

    if is_collision {
        entity
    } else {
        Entity {
            position: new_pos,
            size: entity.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;

    use crate::model::Texture;

    use super::*;

    #[test]
    fn test_move_entity() {
        let entity = Entity {
            position: vec2(0.0, 0.0),
            size: 1.0,
        };
        let movement1 = vec2(0.0, 1.0);
        let walls = vec![Wall {
            texture: Texture::Debug,
            start: vec2(-10.0, 1.5),
            end: vec2(10.0, 1.5),
        }];

        let moved1 = move_entity(entity, movement1, &walls);
        assert_eq!(moved1.position, entity.position);

        let movement2 = vec2(0.0, 0.1);
        let moved2 = move_entity(entity, movement2, &walls);

        assert!(moved2.position.y > entity.position.y);
    }
}
