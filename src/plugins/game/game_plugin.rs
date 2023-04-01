use bevy::prelude::{App, Plugin};

use crate::plugins::core::core_plugins::CorePlugin;
use crate::plugins::enemy::enemy_plugins::EnemyPlugin;
use crate::plugins::food::food_plugins::FoodPlugin;
use crate::plugins::levels::level_plugins::LevelPluginGroup;
use crate::plugins::player::player_plugin::PlayerPlugin;
use crate::plugins::states::GameState;
use crate::plugins::ui::ui_plugin::UiPlugins;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();

        app.add_plugins(LevelPluginGroup).add_plugins(UiPlugins);

        app.add_plugin(CorePlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(EnemyPlugin);
    }
}
