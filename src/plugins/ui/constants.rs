use bevy::{
    prelude::Color,
    ui::{PositionType, Size, Style, UiRect, Val},
};

// UI box
pub const UI_BOX_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
    },
    ..Style::DEFAULT
};
pub const UI_BOX_BG_COLOR: Color = Color::rgba(0.65, 0.65, 0.65, 0.5);

// fps
pub const FPS_FONT_SIZE: f32 = 20.0;
pub const FPS_STYLE: Style = Style {
    position: UiRect {
        left: Val::Px(0.0),
        bottom: Val::Px(0.0),
        ..UiRect::DEFAULT
    },
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};

// score
pub const SCORE_FONT_SIZE: f32 = 20.0;
