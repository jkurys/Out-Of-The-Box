use bevy::prelude::*;

use crate::consts::MAIN_MENU_FONT;

use super::{LevelEditorFileName, LevelEditorSaveItem};

pub fn setup_file_name_getter(asset_server: Res<AssetServer>, mut commands: Commands) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn((
            BackgroundColor(Color::BLACK),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
        ))
        .insert(LevelEditorSaveItem)
        .with_children(|parent| {
            parent.spawn((
                Text::new("Please provide the level name"),
                TextFont {
                    font: menu_font.clone(),
                    font_size: 50.,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent
                .spawn((
                    Text::new(""),
                    TextFont {
                        font: menu_font.clone(),
                        font_size: 30.,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ))
                .insert(LevelEditorFileName);
        });
}
