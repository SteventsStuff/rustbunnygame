use bevy::prelude::Component;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthValue;
