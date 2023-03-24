use bevy::{
    prelude::{Handle, Image, Resource},
    text::Font,
};

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub image_handles: Vec<Handle<Image>>,
    pub font_handles: Vec<Handle<Font>>,
}

#[derive(Resource, Debug, Default)]
pub struct FoodStats {
    pub player_ate_count: usize,
    pub enemy_ate_count: usize,
}
