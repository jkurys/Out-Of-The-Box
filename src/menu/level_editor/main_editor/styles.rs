use bevy::prelude::*;

pub const OBJECTS_COMPARTMENT_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(5.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    align_self: AlignSelf::End,
    ..Style::DEFAULT
};

pub const FLOORS_COMPARTMENT_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(5.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    align_self: AlignSelf::Start,
    ..Style::DEFAULT
};

pub const TABS_COMPARTMENT_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(100.0),
        height: Val::Percent(3.0),
    },
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    align_self: AlignSelf::Start,
    ..Style::DEFAULT
};
