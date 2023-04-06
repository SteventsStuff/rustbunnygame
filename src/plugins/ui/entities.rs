use super::{
    components::{FpsText, HealthValue, ScoreText},
    constants::{FPS_FONT_SIZE, FPS_STYLE, SCORE_FONT_SIZE, UI_BOX_BG_COLOR, UI_BOX_STYLE},
};
use bevy::{
    prelude::{
        default, AssetServer, Bundle, Color, Component, Handle, ImageBundle, NodeBundle, Res,
        TextBundle,
    },
    text::{Font, TextSection, TextStyle},
    ui::{Size, Style, UiImage, UiRect, Val, ZIndex},
};

#[derive(Component)]
pub struct UiBox;

impl UiBox {
    pub fn new() -> NodeBundle {
        NodeBundle {
            style: UI_BOX_STYLE,
            // background_color: UI_BOX_BG_COLOR.into(),
            ..default()
        }
    }
}

#[derive(Bundle)]
pub struct ScoreTextEntity {
    tag: ScoreText,

    #[bundle]
    pub text_bundle: TextBundle,
}

#[derive(Bundle)]
pub struct ScoreBoxEntity {
    #[bundle]
    pub text: ScoreTextEntity,
    pub icon: ImageBundle,
}

impl ScoreBoxEntity {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let image = ImageBundle {
            style: Style {
                size: Size {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                },
                ..default()
            },
            image: UiImage {
                texture: asset_server.load("images/ui/star.png"),
                ..default()
            },
            ..default()
        };

        Self {
            icon: image,
            text: ScoreTextEntity {
                tag: ScoreText,
                text_bundle: ScoreBoxEntity::create_text(&asset_server),
            },
        }
    }

    fn create_text(asset_server: &Res<AssetServer>) -> TextBundle {
        let font_handle = asset_server.load("fonts/FiraMono-Medium.ttf");
        let score_text_section = TextSection::new(
            "SCORE: ",
            TextStyle {
                font: font_handle.clone(),
                font_size: SCORE_FONT_SIZE,
                color: Color::WHITE,
            },
        );
        let score_value_section = TextSection::from_style(TextStyle {
            font: font_handle.clone(),
            font_size: SCORE_FONT_SIZE,
            color: Color::BLACK,
        });

        let mut text_entity = TextBundle::from_sections([score_text_section, score_value_section]);
        text_entity.style.margin = UiRect {
            left: Val::Px(5.0),
            ..default()
        };

        text_entity
    }
}

#[derive(Bundle)]
pub struct FpsTextEntity {
    tag: FpsText,
    #[bundle]
    text_bundle: TextBundle,
}

impl FpsTextEntity {
    pub fn new(font: &Handle<Font>) -> Self {
        let fps_text_section = TextSection::new(
            "FPS: ",
            TextStyle {
                font: font.clone(),
                font_size: FPS_FONT_SIZE,
                color: Color::WHITE,
            },
        );
        let fps_value_section = TextSection::from_style(TextStyle {
            font: font.clone(),
            font_size: FPS_FONT_SIZE,
            color: Color::BLACK,
        });

        let mut text = TextBundle::from_sections([fps_text_section, fps_value_section]);
        text.z_index = ZIndex::Global(999);
        text.style = FPS_STYLE;

        Self {
            tag: FpsText,
            text_bundle: text,
        }
    }
}

#[derive(Component)]
pub struct HealthHUD {
    tag: HealthValue,
    pub health_value: Vec<ImageBundle>,
}

impl HealthHUD {
    pub fn new(max_health: i8, asset_server: &Res<AssetServer>) -> Self {
        let mut health_vec: Vec<ImageBundle> = Vec::new();

        for _ in 1..=max_health {
            let heart_icon = ImageBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                    },
                    ..default()
                },
                image: UiImage {
                    texture: asset_server.load("images/ui/heart.png"),
                    ..default()
                },
                ..default()
            };
            health_vec.push(heart_icon);
        }

        HealthHUD {
            tag: HealthValue,
            health_value: health_vec,
        }
    }
}
