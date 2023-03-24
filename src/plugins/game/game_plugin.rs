use bevy::prelude::{App, Plugin};

use crate::plugins::devtools::devtools_plugins::DevtoolsPluginGroup;
use crate::plugins::enemy::enemy_plugins::EnemyPlugin;
use crate::plugins::food::food_plugins::FoodPlugin;
use crate::plugins::levels::level_plugins::LevelPluginGroup;
use crate::plugins::player::player_plugin::PlayerPlugin;
use crate::plugins::states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();

        app.add_plugins(DevtoolsPluginGroup)
            .add_plugins(LevelPluginGroup)
            .add_plugin(PlayerPlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(EnemyPlugin);
    }
}
