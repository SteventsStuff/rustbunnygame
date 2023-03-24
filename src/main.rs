mod plugins;

use bevy::{app::App, DefaultPlugins};

use crate::plugins::game::game_plugin::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
