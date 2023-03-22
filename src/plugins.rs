use bevy::{asset::LoadState, prelude::*};

use crate::resources;
use crate::states::AppState;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::FoodStats::new())
            .init_resource::<resources::SceneAssets>()
            .add_system(init_assets_loading_system.in_schedule(OnEnter(AppState::Setup)))
            .add_system(check_assets_loading.in_set(OnUpdate(AppState::Setup)));
    }
}

fn init_assets_loading_system(
    asset_server: Res<AssetServer>,
    mut scene_assets: ResMut<resources::SceneAssets>,
) {
    let bg_images_handle = asset_server.load_folder("bg").expect("no such folder");
    let food_images_handle = asset_server.load_folder("food").expect("no such folder");
    let player_images_handle = asset_server.load_folder("player").expect("no such folder");
    let sprites_images_handle = asset_server.load_folder("sprites").expect("no such folder");

    let mut image_handles: Vec<Handle<Image>> = vec![];
    image_handles.extend(bg_images_handle.iter().map(|h| h.clone().typed::<Image>()));
    image_handles.extend(
        sprites_images_handle
            .iter()
            .map(|h| h.clone().typed::<Image>()),
    );
    image_handles.extend(
        food_images_handle
            .iter()
            .map(|h| h.clone().typed::<Image>()),
    );
    image_handles.extend(
        player_images_handle
            .iter()
            .map(|h| h.clone().typed::<Image>()),
    );

    scene_assets.image_handles = image_handles;
}

fn check_assets_loading(
    asset_server: Res<AssetServer>,
    scene_assets: Res<resources::SceneAssets>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let LoadState::Loaded = asset_server
        .get_group_load_state(scene_assets.image_handles.iter().map(|handle| handle.id()))
    {
        info!("assets loaded!");
        next_state.set(AppState::Finished);
    }
}
