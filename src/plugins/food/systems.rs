use bevy::{
    prelude::{info, AssetServer, Commands, EventReader, Handle, Image, Query, Res, Vec3, With},
    window::{PrimaryWindow, Window},
};
use rand::Rng;

use crate::plugins::events::SpawnNewFoodEvent;

use super::{
    constants::{FOOD_LAYER_Z_INDEX, FOOD_SIZE, FOOD_SPAWN_AMOUNT},
    entities::FoodEntity,
    utils,
};

pub fn spawn_food_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    place_food(&mut commands, &asset_server, &window_query);
}

pub fn add_new_food(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ev_spawn_food: EventReader<SpawnNewFoodEvent>,
) {
    for _ in ev_spawn_food.iter() {
        info!("spawning food by event!");
        place_food(&mut commands, &asset_server, &window_query);
    }
}

fn place_food(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut rnd = rand::thread_rng();

    let screen_padding = FOOD_SIZE * 1.5f32;
    let (x_min, x_max) = (
        (window.width() / -2.0) + screen_padding,
        (window.width() / 2.0) - screen_padding,
    );
    let (y_min, y_max) = (
        (window.height() / -2.0) + screen_padding,
        (window.height() / 2.0) - screen_padding,
    );

    let mut food_items: Vec<FoodEntity> = vec![];
    for _ in 1..=FOOD_SPAWN_AMOUNT {
        let food_texture_handle: Handle<Image> = asset_server.load(utils::get_food_sprite_path());
        let pos_x = rnd.gen_range(x_min..=x_max);
        let pos_y = rnd.gen_range(y_min..=y_max);
        let position = Vec3::new(pos_x, pos_y, FOOD_LAYER_Z_INDEX);

        let food = FoodEntity::new(food_texture_handle, position);
        food_items.push(food);
    }
    commands.spawn_batch(food_items);
}
