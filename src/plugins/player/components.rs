use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerType;

#[derive(Component, Default)]
pub struct NoDamageFrames {
    pub no_damage: bool,
    pub frames_left: i8,
}
