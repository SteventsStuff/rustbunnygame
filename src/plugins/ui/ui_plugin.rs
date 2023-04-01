use bevy::{
    app::PluginGroupBuilder,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin, PluginGroup},
};

use crate::plugins::states::GameState;

use super::systems::{fps_value_update_system, setup};

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(HUDPlugin)
            .add(FPSPlugin)
    }
}

struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)));
    }
}
struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fps_value_update_system);
    }
}
