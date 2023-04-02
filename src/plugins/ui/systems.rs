//! This example illustrates the various features of Bevy UI.

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::plugins::resources;

use super::{
    components::{FpsText, ScoreText},
    constants::UI_BOX_BG_COLOR,
    entities::{FpsTextEntity, ScoreBoxEntity, UiBox, HealthHUD},
};

pub fn setup_hub(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(UiBox::new()).with_children(|parent| {
        // score
        parent
            .spawn(NodeBundle {
                style: Style {
                    size: Size {
                        height: Val::Px(20.0),
                        ..default()
                    },
                    margin: UiRect {
                        top: Val::Px(5.0),
                        left: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: UI_BOX_BG_COLOR.into(),
                ..default()
            })
            .with_children(|parent| {
                let score_entity = ScoreBoxEntity::new(&asset_server);
                // icon
                parent.spawn(score_entity.icon);
                // value
                parent.spawn(score_entity.text);
            });
        
        // health
        parent.spawn(NodeBundle {
            style: Style {
                size: Size {
                    height: Val::Px(30.0),
                    ..default()
                },
                margin: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                position: UiRect {
                    right: Val::Px(0.0),
                    ..default() 
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: UI_BOX_BG_COLOR.into(),
            ..default()
        }).with_children(|parent| {
            let health_hud = HealthHUD::new(3, &asset_server);
            for heart in health_hud.health_value{
                parent.spawn(heart);
            }
        });
    });
}

pub fn update_ui_score_value(
    game_score: ResMut<resources::FoodStats>,
    mut q_score_text: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(mut text) = q_score_text.get_single_mut() {
        text.sections[1].value = format!("{}", game_score.player_ate_count);
    }
}

pub fn setup_fps(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn(FpsTextEntity::new(&font_handle));
}

pub fn fps_value_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let mut text = query.get_single_mut().unwrap();
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[1].value = format!("{value:.2}");
        }
    }
}
