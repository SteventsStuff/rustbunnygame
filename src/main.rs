mod plugins;

use bevy::app::App;

use crate::plugins::game::game_plugin::GamePlugin;

fn main() {
    App::new().add_plugin(GamePlugin).run();
}
