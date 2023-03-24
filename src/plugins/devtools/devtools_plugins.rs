use bevy::{
    app::PluginGroupBuilder,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, Plugin, PluginGroup},
};

use super::systems::{fps_screen_message_setup, fps_value_update_system};

pub struct DevtoolsPluginGroup;

impl PluginGroup for DevtoolsPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(FpsScreenTextPlugin)
    }
}

struct FpsScreenTextPlugin;

impl Plugin for FpsScreenTextPlugin {
    fn build(&self, app: &mut App) {
        // todo: add state (???)
        app.add_startup_system(fps_screen_message_setup)
            .add_system(fps_value_update_system);
    }
}
