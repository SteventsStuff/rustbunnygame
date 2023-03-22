mod bundles;
mod componetns;
mod consts;
mod plugins;
mod resources;
mod states;
mod systems;
mod utils;

use bevy::{
    app::App,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{IntoSystemAppConfigs, IntoSystemConfigs, OnEnter, OnUpdate},
    DefaultPlugins,
};
use plugins as custome_plugins;
use states::AppState;
use systems::{
    animate_sprite, basic_setup, check_enemy_collision_system, check_player_collision_system,
    confine_enemy_movement_system, confine_player_movement_system, draw_background_system,
    enemy_movement_system, player_movement_system, spawn_enemy, spawn_food_system,
    spawn_player_system, text_setup, text_update_system,
};

fn main() {
    // NOTE: did not find a way to create a window without DefaultPlugins (by using only WindowPlugin)
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(custome_plugins::ResourcesPlugin)
        .add_systems(
            (
                basic_setup,
                draw_background_system,
                spawn_player_system,
                spawn_food_system,
                spawn_enemy,
                text_setup,
            )
                .in_schedule(OnEnter(AppState::Finished)),
        )
        .add_systems(
            (
                player_movement_system,
                confine_player_movement_system,
                enemy_movement_system,
                confine_enemy_movement_system,
                check_player_collision_system,
                check_enemy_collision_system,
                animate_sprite,
                text_update_system,
            )
                .in_set(OnUpdate(AppState::Finished)),
        )
        .run();
}
