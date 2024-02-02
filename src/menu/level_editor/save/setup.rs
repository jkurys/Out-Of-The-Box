use bevy::prelude::*;

use crate::consts::MAIN_MENU_FONT;

use super::{LevelEditorFileName, LevelEditorSaveItem};

pub fn setup_file_name_getter(asset_server: Res<AssetServer>, mut commands: Commands) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility::Visible,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(LevelEditorSaveItem)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Please provide the level name",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 50.,
                    color: Color::WHITE,
                },
            ));
            parent
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: menu_font.clone(),
                        font_size: 30.,
                        color: Color::WHITE,
                    },
                ))
                .insert(LevelEditorFileName);
        });
}
