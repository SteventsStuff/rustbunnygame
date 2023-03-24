use bevy::{
    asset::LoadState,
    prelude::{
        default, info, AssetServer, Camera2dBundle, Commands, Handle, Image, NextState, Query, Res,
        ResMut, Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    text::Font,
    window::{PrimaryWindow, Window},
};

use crate::plugins::{resources::SceneAssets, states::GameState};

use super::constants::BG_LAYER_Z_INDEX;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn draw_background_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, BG_LAYER_Z_INDEX),
            ..default()
        },
        texture: asset_server.load("images/bg/bg2.png"),
        ..default()
    });
}

pub fn init_assets_loading_system(
    asset_server: Res<AssetServer>,
    mut scene_assets: ResMut<SceneAssets>,
) {
    let image_handles = asset_server.load_folder("images").expect("no such folder");
    let font_handles = asset_server.load_folder("fonts").expect("no such folder");

    scene_assets.image_handles =
        Vec::<Handle<Image>>::from_iter(image_handles.iter().map(|h| h.clone().typed::<Image>()));
    scene_assets.font_handles =
        Vec::<Handle<Font>>::from_iter(font_handles.iter().map(|h| h.clone().typed::<Font>()));
}

pub fn check_assets_loading(
    asset_server: ResMut<AssetServer>,
    scene_assets: Res<SceneAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let images_load_state = asset_server
        .get_group_load_state(scene_assets.image_handles.iter().map(|handle| handle.id()));
    let fonts_load_state = asset_server
        .get_group_load_state(scene_assets.font_handles.iter().map(|handle| handle.id()));
    let statuses = vec![images_load_state, fonts_load_state];

    if statuses.iter().all(|s| *s == LoadState::Loaded) {
        info!("all assets loaded!");
        next_state.set(GameState::InGame);
    }

    if let LoadState::Loading = images_load_state {
        info!("loading images...");
    }

    if let LoadState::Loading = fonts_load_state {
        info!("loading fonts...");
    }
}
