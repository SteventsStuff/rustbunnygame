use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::plugins::resources;

use super::{
    components::{FpsText, ScoreText},
    constants::{SCORE_STYLE, UI_BOX_BG_COLOR},
    entities::{FpsTextEntity, ScoreBoxEntity, UiBox},
};

pub fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(UiBox::new()).with_children(|parent| {
        // score
        add_score_hud_element(parent, &asset_server);
        // health
        // add_health_hud_element(parent, &asset_server);
    });
}

fn add_score_hud_element(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: SCORE_STYLE,
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
}

// fn add_health_hud_element(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
//     parent
//         .spawn(NodeBundle {
//             background_color: UI_BOX_BG_COLOR.into(),
//             ..default()
//         })
//         .with_children(|parent| {
//             let health_hud = HealthHUD::new(3, &asset_server);
//             for heart in health_hud.health_value {
//                 parent.spawn(heart);
//             }
//         });
// }

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
