use crate::{
    math::check_circles_collide,
    model::{key_object::KeyObject, GameEvent, Player},
};

pub fn check_pickup_key(player: &Player, keys: Vec<KeyObject>) -> (Vec<KeyObject>, Vec<GameEvent>) {
    let picked_up: Vec<_> = keys
        .into_iter()
        .map(|key| {
            (
                check_circles_collide(
                    key.entity.position,
                    key.entity.size,
                    player.entity.position,
                    player.entity.size,
                ),
                key,
            )
        })
        .collect();

    let events = picked_up
        .iter()
        .filter_map(|(picked_up, _)| {
            if *picked_up {
                Some(GameEvent::PickUpKey)
            } else {
                None
            }
        })
        .collect();

    let remaining_keys = picked_up
        .into_iter()
        .filter_map(|(picked_up, key)| if picked_up { None } else { Some(key) })
        .collect();

    (remaining_keys, events)
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec2;

    use crate::model::Entity;

    use super::*;

    #[test]
    fn test_check_pickup_key() {
        let player = Player {
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: 1.0,
            },
            look: vec2(0.0, 0.0),
        };

        let key = KeyObject {
            entity: Entity {
                position: vec2(0.0, 0.5),
                size: 1.0,
            },
        };

        let keys = vec![key];

        let (new_keys, events) = check_pickup_key(&player, keys.clone());
        assert_eq!(events.len(), 1);
        assert_eq!(new_keys.len(), 0);

        let player_far = Player {
            entity: Entity {
                position: vec2(10.0, 10.0),
                size: 1.0,
            },
            look: vec2(0.0, 0.0),
        };

        let (new_keys_far, events_far) = check_pickup_key(&player_far, keys);
        assert_eq!(events_far.len(), 0);
        assert_eq!(new_keys_far.len(), 1);
    }
}
