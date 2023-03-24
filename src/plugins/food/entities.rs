use bevy::{
    prelude::{default, Bundle, Handle, Image, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use crate::plugins::components::Collider;

use super::{components::FoodType, constants::FOOD_SIZE};

#[derive(Bundle)]
pub struct FoodEntity {
    entity_type: FoodType,
    collider: Collider,

    #[bundle]
    _sprite: SpriteBundle,
}

impl FoodEntity {
    pub fn new(texture_handle: Handle<Image>, pos: Vec3) -> Self {
        let food_size = Vec2::new(FOOD_SIZE, FOOD_SIZE);

        Self {
            entity_type: FoodType,
            collider: Collider,

            _sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(food_size),
                    ..default()
                },
                texture: texture_handle,
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                ..default()
            },
        }
    }
}
