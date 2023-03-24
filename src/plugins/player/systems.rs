use bevy::{
    prelude::{
        info, AssetServer, Commands, Entity, Handle, Image, Input, KeyCode, Query, Res, ResMut,
        Transform, Vec3, With, Without,
    },
    sprite::{collide_aabb::collide, Sprite},
    time::Time,
    window::{PrimaryWindow, Window},
};

use crate::plugins::{components::Collider, food::components::FoodType, resources};

use super::{
    components::PlayerType,
    constants::{PLAYER_SIZE, PLAYER_SPEED},
    entities::PlayerEntity,
};

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("images/player/player.png");
    let player = PlayerEntity::new(texture_handle);
    commands.spawn(player);
    info!("player spawned");
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerType>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction: Vec3 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement_system(
    mut player_query: Query<&mut Transform, With<PlayerType>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut enemy_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;

        let x_min = window.width() / -2.0 + half_player_size;
        let x_max = window.width() / 2.0 - half_player_size;
        let y_min = window.height() / -2.0 + half_player_size;
        let y_max = window.height() / 2.0 - half_player_size;

        // println!("x_min: {}, x_max: {}, y_min: {}, y_max: {}", x_min, x_max, y_min, y_max);

        let mut currect_position = enemy_transform.translation;
        // println!("currecnt position: {}", currect_position);

        // TODO: make animation?
        // fix X postions
        if currect_position.x < x_min {
            currect_position.x = x_min;
        } else if currect_position.x > x_max {
            currect_position.x = x_max;
        }

        // fix Y postions
        if currect_position.y < y_min {
            currect_position.y = y_min;
        } else if currect_position.y > y_max {
            currect_position.y = y_max;
        }

        enemy_transform.translation = currect_position;
    }
}

// todo: make ALL collisoin stuff work on events via a separate plugin (later)
// todo: resources such as score and end game also msut be done via events (later)
pub fn check_player_collision_system(
    mut commands: Commands,
    player_query: Query<(&mut Transform, &Sprite), With<PlayerType>>,
    food_query: Query<
        (Entity, &mut Transform, &Sprite),
        (Without<PlayerType>, With<Collider>, With<FoodType>),
    >,
    mut game_score: ResMut<resources::FoodStats>,
) {
    if let Ok((player_transform, player_sprite)) = player_query.get_single() {
        let player_pos = player_transform.translation;
        let player_size = player_sprite.custom_size.unwrap();

        for (food_entity, food_transform, foot_sprite) in food_query.into_iter() {
            let food_pos = food_transform.translation;
            let food_size = foot_sprite.custom_size.unwrap();

            let collision = collide(player_pos, player_size, food_pos, food_size);
            if let Some(_collision) = collision {
                commands.entity(food_entity).despawn();
                game_score.player_ate_count += 1;
                info!("Food score: {:?}", game_score);
            }
        }
    }
}
