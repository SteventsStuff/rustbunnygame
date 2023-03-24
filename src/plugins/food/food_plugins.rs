use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, Plugin};

use crate::plugins::states::GameState;

use super::systems::spawn_food_system;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_food_system.in_schedule(OnEnter(GameState::InGame)));
    }
}
