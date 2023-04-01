use bevy::{
    prelude::{default, Bundle, Handle, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};

use crate::plugins::components::Collider;

use super::{
    components::{AnimationIndices, AnimationTimer, Direction, EnemyType},
    constants::{ENEMY_LAYER_Z_INDEX, ENEMY_SIZE},
};

#[derive(Bundle)]
pub struct EnemyEntity {
    entity_type: EnemyType,
    collider: Collider,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    // todo: remove this shit
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
                    translation: Vec3::new(
                        direction.x + 150.0,
                        direction.y + 150.0,
                        ENEMY_LAYER_Z_INDEX,
                    ),
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: animation_indices.first,
                    custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
                ..default()
            },
        }
    }
}
