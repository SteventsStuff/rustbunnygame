use bevy::prelude::{Handle, Image, Resource};

#[derive(Resource)]
pub struct FoodStats {
    pub eaten_count: usize,
}

impl FoodStats {
    pub fn new() -> Self {
        Self { eaten_count: 0 }
    }
}

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub image_handles: Vec<Handle<Image>>,
}
