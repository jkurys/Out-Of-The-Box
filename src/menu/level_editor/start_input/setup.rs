use bevy::prelude::*;

use crate::{consts::MAIN_MENU_FONT, menu::level_editor::LevelEditorItem};

use super::{LevelEditorInputNumber, LevelEditorStartingPrompt};

pub fn setup_level_editor(asset_server: Res<AssetServer>, mut commands: Commands) {
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
        .insert(LevelEditorItem)
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("Please provide the level width"),
                    TextFont {
                        font: menu_font.clone(),
                        font_size: 50.,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ))
                .insert(LevelEditorStartingPrompt);
            parent
                .spawn((
                    Text::new("0"),
                    TextFont {
                        font: menu_font.clone(),
                        font_size: 50.,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ))
                .insert(LevelEditorInputNumber);
        });
}
