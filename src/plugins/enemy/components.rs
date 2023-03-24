use bevy::{
    prelude::{Component, Deref, DerefMut, Vec3},
    time::Timer,
};

#[derive(Component)]
pub struct EnemyType;

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Direction(pub Vec3);
