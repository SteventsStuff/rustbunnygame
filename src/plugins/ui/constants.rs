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

// HUD general
pub const HUD_ELEMENT_MARGIN: UiRect = UiRect {
    top: Val::Px(5.0),
    right: Val::Px(5.0),
    ..UiRect::DEFAULT
};

// HUD health
pub const HEALTH_STYLE: Style = Style {
    size: Size {
        height: Val::Px(30.0),
        ..Size::DEFAULT
    },
    margin: HUD_ELEMENT_MARGIN,
    position: UiRect {
        right: Val::Px(0.0),
        ..UiRect::DEFAULT
    },
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};

// HUD score
pub const SCORE_FONT_SIZE: f32 = 20.0;
pub const SCORE_STYLE: Style = Style {
    size: Size {
        height: Val::Px(20.0),
        ..Size::DEFAULT
    },
    margin: HUD_ELEMENT_MARGIN,
    ..Style::DEFAULT
};
