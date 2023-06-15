use bevy::prelude::*;

use crate::{consts::MAIN_MENU_FONT, menu::level_editor::LevelEditorItem};

use super::{LevelEditorStartingPrompt, LevelEditorInputNumber};

pub fn setup_level_editor(asset_server: Res<AssetServer>, mut commands: Commands) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility::Visible,
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Please provide the level width",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 50.,
                    color: Color::WHITE,
                },
            )).insert(LevelEditorStartingPrompt);
            parent.spawn(TextBundle::from_section(
                "0",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 50.,
                    color: Color::WHITE,
                },
            )).insert(LevelEditorInputNumber);
        });
}