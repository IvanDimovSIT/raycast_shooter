use std::time::Duration;

use crate::{constants::{ENEMY_ATTACK_RANGE, ENEMY_DPS}, model::Animation, service::enemy::{enemy_can_attack_player, move_enemy}};
use macroquad::math::{vec2, Vec2};
use uuid::Uuid;

use crate::{
    constants::{ENEMY_ANIMATION_SPEED, ENEMY_DRAW_SIZE_MOD, ENEMY_HP, ENEMY_SIZE},
    draw::{calculate_vertical_offset, select_animation_texture, sprite_2d::Sprite2D},
};

use super::{Enemy, Entity, GameEvent, Player, Texture, Wall};

#[derive(Debug, Clone)]
pub struct MeleeEnemy {
    pub id: Uuid,
    pub entity: Entity,
    pub hp: f32,
}
impl Sprite2D for MeleeEnemy {
    fn get_position(&self) -> Vec2 {
        self.entity.position
    }

    fn get_vertical_offset(&self, time_ellapsed: &Duration) -> f32 {
        calculate_vertical_offset(300, self.get_size(), -0.1, 0.02, time_ellapsed)
    }

    fn get_size(&self) -> f32 {
        self.entity.size * ENEMY_DRAW_SIZE_MOD
    }

    fn get_texture(&self, time_ellapsed: &Duration) -> Texture {
        select_animation_texture(
            &Animation::Enemy.get_textures(),
            ENEMY_ANIMATION_SPEED,
            time_ellapsed,
        )
    }
}
impl Enemy for MeleeEnemy {
    fn update(&self, player: &Player, walls: &[Wall], delta: f32) -> (Box<dyn Enemy>, Vec<GameEvent>) {
        let new_entity = move_enemy(player, self.entity, walls, delta);
        let events = if enemy_can_attack_player(&new_entity, ENEMY_ATTACK_RANGE, player, walls) {
            vec![GameEvent::PlayerTakeDamage(delta*ENEMY_DPS)]
        }else{
            vec![]
        };

        (Box::new(MeleeEnemy{entity: new_entity, ..*self}), events)
    }
    
    fn as_sprite(&self) -> &dyn Sprite2D {
        self as &dyn Sprite2D
    }
    
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn take_damage(&self, damage: f32) -> Box<dyn Enemy> {
        Box::new(
            MeleeEnemy {
                hp: self.hp - damage,
                ..*self
            }
        )
    }
    
    fn get_hp(&self) -> f32 {
        self.hp
    }
}
impl Default for MeleeEnemy {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            entity: Entity {
                position: vec2(0.0, 0.0),
                size: ENEMY_SIZE,
            },
            hp: ENEMY_HP,
        }
    }
}
