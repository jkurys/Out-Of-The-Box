use bevy::prelude::*;

pub const IMAGE_STYLE: Style = Style {
    size: Size {
        width: Val::Px(100.0),
        height: Val::Px(100.0),
    },
    flex_direction: FlexDirection::ColumnReverse,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    ..Style::DEFAULT
};

pub const BACKGROUND_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    ..Style::DEFAULT
};
