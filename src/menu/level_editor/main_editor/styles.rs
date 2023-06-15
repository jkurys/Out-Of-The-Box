use bevy::prelude::*;

pub const COLUMN_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const BOARD_COMPARTMENT_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(70.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const BOARD_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(70.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const TABS_COMPARTMENT_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::FlexStart,
    size: Size {
        width: Val::Percent(100.),
        height: Val::Percent(5.),
    },
    ..Style::DEFAULT
};

pub const TABS_STYLE: Style = Style {
    size: Size {
        height: Val::Percent(100.),
        width: Val::Percent(10.),
    },
    ..Style::DEFAULT
};

pub const PLUS_STYLE: Style = Style {
    size: Size {
        width: Val::Px(20.),
        height: Val::Px(20.),
    },
    margin: UiRect {
        left: Val::Px(10.),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub const RIGHT_COMPARTMENT_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(30.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    ..Style::DEFAULT
};

pub const OBJECTS_COMPARTMENT_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(50.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    ..Style::DEFAULT
};

pub const FLOORS_COMPARTMENT_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(50.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    ..Style::DEFAULT
};