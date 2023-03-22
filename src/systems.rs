use bevy::{
    ecs::system::Commands,
    prelude::{
        default, info, AssetServer, Assets, Camera2dBundle, Color, Entity, Handle, Image, Input,
        KeyCode, ParamSet, Query, Res, ResMut, TextBundle, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{collide_aabb::collide, Sprite, SpriteBundle, TextureAtlas, TextureAtlasSprite},
    text::{TextAlignment, TextSection, TextStyle, Text},
    time::{Time, Timer, TimerMode},
    ui::{PositionType, Style, UiRect, Val},
    window::{PrimaryWindow, Window}, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};
use rand::{self, Rng};

use crate::{
    bundles::{EnemyEntity, FoodEntity, PlayerEntity},
    componetns::{
        AnimationIndices, AnimationTimer, Collider, ColorText, Direction, EnemyType, FoodType,
        FpsText, PlayerType,
    },
    consts, resources, utils,
};

pub fn basic_setup(mut commands: Commands) {
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
            translation: Vec3::new(0.0, 0.0, consts::BG_LAYER_Z_INDEX),
            ..default()
        },
        texture: asset_server.load("bg/bg2.png"),
        ..default()
    });
}

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("player/player.png");
    let player = PlayerEntity::new(texture_handle);
    commands.spawn(player);
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    let animation_timer = AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));

    let init_direction = utils::generate_direction();
    info!("direction: {:?}", init_direction);
    commands.spawn(EnemyEntity::new(
        animation_indices,
        texture_atlas_handle,
        animation_timer,
        init_direction,
    ));
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn spawn_food_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut rnd = rand::thread_rng();

    let screen_padding = consts::FOOD_SIZE * 1.5f32;
    let (x_min, x_max) = (
        (window.width() / -2.0) + screen_padding,
        (window.width() / 2.0) - screen_padding,
    );
    let (y_min, y_max) = (
        (window.height() / -2.0) + screen_padding,
        (window.height() / 2.0) - screen_padding,
    );

    let mut food_items: Vec<FoodEntity> = vec![];
    for _ in 1..=consts::FOOD_SPAWN_AMOUNT {
        let food_texture_handle: Handle<Image> = asset_server.load(utils::get_food_sprite_path());
        let pos_x = rnd.gen_range(x_min..=x_max);
        let pos_y = rnd.gen_range(y_min..=y_max);
        let position = Vec3::new(pos_x, pos_y, consts::FOOD_LAYER_Z_INDEX);

        let food = FoodEntity::new(food_texture_handle, position);
        food_items.push(food);
    }
    commands.spawn_batch(food_items);
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
        transform.translation += direction * consts::PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn enemy_movement_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Direction), With<EnemyType>>,
) {
    if let Ok((mut transform, direction)) = player_query.get_single_mut() {
        let mut directoin_delta = Vec3::ZERO;

        if direction.0.x > 0.0 {
            directoin_delta += Vec3::new(1.0, 0.0, 0.0);
        }
        if direction.0.x <= 0.0 {
            directoin_delta += Vec3::new(-1.0, 0.0, 0.0);
        }
        if direction.0.y > 0.0 {
            directoin_delta += Vec3::new(0.0, 1.0, 0.0);
        }
        if direction.0.y <= 0.0 {
            directoin_delta += Vec3::new(0.0, -1.0, 0.0);
        }

        // info!("directoin_delta: {}", directoin_delta);
        transform.translation += directoin_delta * consts::ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement_system(
    mut player_query: Query<&mut Transform, With<PlayerType>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut enemy_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = consts::PLAYER_SIZE / 2.0;

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

pub fn confine_enemy_movement_system(
    mut enemy_query: Query<(&mut Transform, &mut Direction), With<EnemyType>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut enemy_transform, mut direction)) = enemy_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_enemy_size = consts::ENEMY_SIZE / 2.0;

        let x_min = window.width() / -2.0 + half_enemy_size;
        let x_max = window.width() / 2.0 - half_enemy_size;
        let y_min = window.height() / -2.0 + half_enemy_size;
        let y_max = window.height() / 2.0 - half_enemy_size;

        let currect_position = enemy_transform.translation;

        // fix X postions
        if currect_position.x < x_min || currect_position.x > x_max {
            direction.0.x = direction.0.x * -1.0;
        }
        // fix Y postions
        if currect_position.y < y_min || currect_position.y > y_max {
            direction.0.y = direction.0.y * -1.0;
        }

        enemy_transform.translation = currect_position;
    }
}

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

            // println!("food pos: {}", food_pos);
            let collision = collide(player_pos, player_size, food_pos, food_size);
            if let Some(_collision) = collision {
                // print!("collided!!!!");
                commands.entity(food_entity).despawn();
                game_score.eaten_count += 1;
                info!("Food score: {}", game_score.eaten_count);
            }
        }
    }
}

pub fn check_enemy_collision_system(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut Transform, &TextureAtlasSprite), With<EnemyType>>,
        Query<(Entity, &mut Transform, &Sprite), (With<Collider>, With<PlayerType>)>,
        Query<(Entity, &mut Transform, &Sprite), (With<Collider>, With<FoodType>)>,
    )>,
) {
    let enemy = set.p0();
    let (enemy_transform, enemy_sprite) = enemy.single();
    let enemy_pos = enemy_transform.translation;
    let enemy_size = enemy_sprite.custom_size.unwrap();

    let p = set.p1();
    let player = p.get_single();

    if let Ok(player) = player {
        let (player_entity, player_transform, player_sprite) = player;
        let player_pos = player_transform.translation;
        let player_size = player_sprite.custom_size.unwrap();

        let collison = collide(enemy_pos, enemy_size, player_pos, player_size);
        if let Some(_collison) = collison {
            commands.entity(player_entity).despawn();
            info!("Player was killed");
        }
    }

    for (food_entity, food_transform, foot_sprite) in set.p2().into_iter() {
        let food_pos = food_transform.translation;
        let food_size = foot_sprite.custom_size.unwrap();

        // println!("food pos: {}", food_pos);
        let collision = collide(enemy_pos, enemy_size, food_pos, food_size);
        if let Some(_collision) = collision {
            // print!("collided!!!!");
            commands.entity(food_entity).despawn();
            info!("Enemy ate food!");
        }
    }
}

pub fn text_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));
}

pub fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}