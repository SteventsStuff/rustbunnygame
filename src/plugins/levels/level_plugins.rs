use bevy::{
    app::PluginGroupBuilder,
    prelude::{
        App, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfig, OnEnter, OnExit,
        OnUpdate, Plugin, PluginGroup,
    },
};

use crate::plugins::{
    resources::{FoodStats, SceneAssets},
    states::GameState,
};

use super::systems::{
    check_assets_loading, draw_background_system, init_assets_loading_system, spawn_camera,
};

pub struct LevelPluginGroup;

impl PluginGroup for LevelPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(LevelOne)
    }
}

struct LevelOne;

impl Plugin for LevelOne {
    fn build(&self, app: &mut App) {
        // todo: add level state (???)
        app.insert_resource(SceneAssets::default())
            .insert_resource(FoodStats::default());

        app.add_system(init_assets_loading_system.in_schedule(OnEnter(GameState::InLoadingLevel)))
            .add_system(check_assets_loading.in_set(OnUpdate(GameState::InLoadingLevel)))
            .add_systems(
                (draw_background_system, spawn_camera)
                    .in_schedule(OnExit(GameState::InLoadingLevel)),
            );
    }
}
