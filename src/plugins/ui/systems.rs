//! This example illustrates the various features of Bevy UI.

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::{
    components::FpsText,
    entities::{FpsTextEntity, UiBox},
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn(UiBox::new()).with_children(|parent| {
        parent.spawn(FpsTextEntity::new(&font_handle));
    });
}

pub fn fps_value_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}