use bevy::prelude::{App, DefaultPlugins, Plugin, PluginGroup};
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bunny game".to_string(),
                ..default()
            }),
            ..default()
        }));
    }
}
