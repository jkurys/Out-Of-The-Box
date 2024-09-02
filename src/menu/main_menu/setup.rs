use bevy::prelude::*;

use crate::{consts::MAIN_MENU_FONT, menu::spawn_button};

use super::{MainMenuItem, MenuItemType};

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    let background = asset_server.load("textures/menu_background.png");
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
        .insert(MainMenuItem)
        .insert(ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            image: background.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "MAIN MENU",
                    TextStyle {
                        font_size: 50.0,
                        // color: Color::srgba(0., 0., 0., 0.),
                        color: Color::rgba(0., 0., 0., 0.),
                        font: menu_font.clone(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
            spawn_button(
                parent,
                MenuItemType::LevelSelect,
                menu_font.clone(),
                "Level Select",
            );
            spawn_button(
                parent,
                MenuItemType::SpriteSelect,
                menu_font.clone(),
                "Sprite Select",
            );
            spawn_button(
                parent,
                MenuItemType::LevelEditor,
                menu_font.clone(),
                "Level Editor",
            );
            spawn_button(parent, MenuItemType::Exit, menu_font.clone(), "Exit");
        });
}
