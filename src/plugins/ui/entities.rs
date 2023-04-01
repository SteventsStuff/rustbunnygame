use super::{components::FpsText, constants::FPS_FONT_SIZE};
use bevy::{
    prelude::{default, Bundle, Color, Component, Handle, NodeBundle, TextBundle},
    text::{Font, TextSection, TextStyle},
    ui::{Size, Style, Val, ZIndex},
};

#[derive(Component)]
pub struct UiBox;

impl UiBox {
    pub fn new() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                ..default()
            },
            background_color: Color::rgba(0.65, 0.65, 0.65, 0.5).into(),
            ..default()
        }
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

        Self {
            tag: FpsText,
            text_bundle: text,
        }
    }
}
