use bevy::prelude::{App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin};

use crate::plugins::states::GameState;

use super::systems::{
    animate_sprite, check_enemy_collision_system, confine_enemy_movement_system,
    enemy_movement_system, spawn_enemy,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy.in_schedule(OnEnter(GameState::InGame)))
            .add_systems(
                (
                    animate_sprite,
                    enemy_movement_system,
                    confine_enemy_movement_system,
                    check_enemy_collision_system,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}
