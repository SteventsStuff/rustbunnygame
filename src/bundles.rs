use bevy::{
    prelude::{default, Bundle, Handle, Image, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};

use crate::componetns::{
    AnimationIndices, AnimationTimer, Collider, Direction, EnemyType, FoodType, PlayerType,
};
use crate::consts;

#[derive(Bundle)]
pub struct PlayerEntity {
    entity_type: PlayerType,
    collider: Collider,

    #[bundle]
    _sprite: SpriteBundle,
}

impl PlayerEntity {
    pub fn new(texture_handle: Handle<Image>) -> Self {
        let player_size = Vec2::new(consts::PLAYER_SIZE, consts::PLAYER_SIZE);

        Self {
            entity_type: PlayerType,
            collider: Collider,

            _sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, consts::PLAYER_LAYER_Z_INDEX),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(player_size),
                    ..default()
                },
                texture: texture_handle,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct FoodEntity {
    entity_type: FoodType,
    collider: Collider,

    #[bundle]
    _sprite: SpriteBundle,
}

impl FoodEntity {
    pub fn new(texture_handle: Handle<Image>, pos: Vec3) -> Self {
        let food_size = Vec2::new(consts::FOOD_SIZE, consts::FOOD_SIZE);

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

#[derive(Bundle)]
pub struct EnemyEntity {
    entity_type: EnemyType,
    collider: Collider,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    direction: Direction,

    #[bundle]
    _sprite: SpriteSheetBundle,
}

impl EnemyEntity {
    pub fn new(
        animation_indices: AnimationIndices,
        texture_atlas_handle: Handle<TextureAtlas>,
        animation_timer: AnimationTimer,
        direction: Vec3,
    ) -> Self {
        Self {
            animation_timer,
            animation_indices: animation_indices.clone(),
            entity_type: EnemyType,
            collider: Collider,
            direction: Direction(direction),

            _sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(direction.x + 150.0, direction.y + 150.0, 0.0),
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: animation_indices.first,
                    custom_size: Some(Vec2::new(consts::ENEMY_SIZE, consts::ENEMY_SIZE)),
                    ..default()
                },
                ..default()
            },
        }
    }
}
