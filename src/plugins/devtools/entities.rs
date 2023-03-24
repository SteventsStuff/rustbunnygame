use bevy::{
    prelude::{Bundle, Color, Handle, TextBundle},
    text::{Font, TextSection, TextStyle},
};

use super::{components::FpsText, constants::FPS_FONT_SIZE};

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

        Self {
            tag: FpsText,
            text_bundle: TextBundle::from_sections([fps_text_section, fps_value_section]),
        }
    }
}
