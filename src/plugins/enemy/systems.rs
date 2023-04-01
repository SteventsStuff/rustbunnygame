use bevy::{
    ecs::system::Commands,
    prelude::{
        info, AssetServer, Assets, Entity, Handle, Image, ParamSet, Query, Res, ResMut, Transform,
        Vec2, Vec3, With,
    },
    sprite::{collide_aabb::collide, Sprite, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
    window::{PrimaryWindow, Window},
};

use crate::plugins::{
    components::Collider,
    enemy::{
        components::{AnimationIndices, AnimationTimer},
        entities::EnemyEntity,
        utils,
    },
    food::components::FoodType,
    player::components::PlayerType,
    resources,
};

use super::{
    components::{Direction, EnemyType},
    constants::{ENEMY_SIZE, ENEMY_SPEED},
};

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/sprites/gabe-idle-run.png");
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
        transform.translation += directoin_delta * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_movement_system(
    mut enemy_query: Query<(&mut Transform, &mut Direction), With<EnemyType>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut enemy_transform, mut direction)) = enemy_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.0;

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

pub fn check_enemy_collision_system(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut Transform, &TextureAtlasSprite), With<EnemyType>>,
        Query<(Entity, &mut Transform, &Sprite), (With<Collider>, With<PlayerType>)>,
        Query<(Entity, &mut Transform, &Sprite), (With<Collider>, With<FoodType>)>,
    )>,
    mut game_score: ResMut<resources::FoodStats>,
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
            game_score.enemy_ate_count += 1;
            info!("Food score: {:?}", game_score);
        }
    }
}
