use bevy::{
    app::PluginGroupBuilder,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin, PluginGroup},
};

use crate::plugins::states::GameState;

use super::systems::{fps_value_update_system, setup_fps, setup_hud, update_ui_score_value};

// NOTE: this idea about all UI in one plugin group is prob a dogshit, but for this peject it is fine, tho (or not lol)
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
        app.add_system(setup_hud.in_schedule(OnEnter(GameState::InGame)))
            .add_system(update_ui_score_value.in_set(OnUpdate(GameState::InGame)));
    }
}

struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_fps)
            .add_system(fps_value_update_system);
    }
}
