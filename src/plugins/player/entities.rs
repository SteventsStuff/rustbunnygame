use bevy::{
    prelude::{default, Bundle, Handle, Image, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use crate::plugins::components::{Collider, PlayerHealth};

use super::{
    components::{PlayerType, NoDamageFrames},
    constants::{PLAYER_LAYER_Z_INDEX, PLAYER_SIZE, PLAYER_MAX_HEALTH},
};

#[derive(Bundle)]
pub struct PlayerEntity {
    entity_type: PlayerType,
    collider: Collider,
    health: PlayerHealth,
    player_no_damage_frames: NoDamageFrames,

    #[bundle]
    _sprite: SpriteBundle,
}

impl PlayerEntity {
    pub fn new(texture_handle: Handle<Image>) -> Self {
        let player_size = Vec2::new(PLAYER_SIZE.into(), PLAYER_SIZE.into());

        Self {
            entity_type: PlayerType,
            collider: Collider,
            health: PlayerHealth(PLAYER_MAX_HEALTH),
            player_no_damage_frames: NoDamageFrames(false),

            _sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, PLAYER_LAYER_Z_INDEX.into()),
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
