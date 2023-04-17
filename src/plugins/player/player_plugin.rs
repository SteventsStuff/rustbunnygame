use bevy::prelude::{App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin};

use crate::plugins::states::GameState;

use super::systems::{
    check_player_collision_system, confine_player_movement_system, player_movement_system,
    spawn_player_system,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_system.in_schedule(OnEnter(GameState::InGame)));
        app.add_systems(
            (
                player_movement_system,
                confine_player_movement_system,
                check_player_collision_system,
            )
                .in_set(OnUpdate(GameState::InGame)),
        );
    }
}
