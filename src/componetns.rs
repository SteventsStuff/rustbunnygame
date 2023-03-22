use bevy::{
    prelude::{Component, Deref, DerefMut, Vec3},
    time::Timer,
};

#[derive(Component)]
pub struct Health(u8);

#[derive(Component)]
pub struct PlayerType;

#[derive(Component)]
pub struct FoodType;

#[derive(Component)]
pub struct EnemyType;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Direction(pub Vec3);

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
pub struct ColorText;
