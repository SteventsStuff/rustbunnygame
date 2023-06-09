use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, Plugin};

use crate::plugins::states::GameState;

use super::systems::{add_new_food, spawn_food_system};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_food_system.in_schedule(OnEnter(GameState::InGame)))
            .add_system(add_new_food);
    }
}
