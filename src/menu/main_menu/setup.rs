use crate::{consts::MAIN_MENU_FONT, menu::spawn_button};
use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;

use super::{MainMenuItem, MenuItemType};

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    let background = asset_server.load("textures/menu_background.png");
    commands
        .spawn((
            BackgroundColor(Color::Srgba(BLACK)),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
        ))
        .insert(MainMenuItem)
        .insert((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ImageNode::new(background.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("MAIN MENU"),
                TextFont {
                    font_size: 50.0,
                    font: menu_font.clone(),
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
            spawn_button(
                parent,
                MenuItemType::LevelSelect,
                menu_font.clone(),
                "Level Select",
                Val::Percent(20.),
                Val::Percent(5.),
            );
            spawn_button(
                parent,
                MenuItemType::SpriteSelect,
                menu_font.clone(),
                "Sprite Select",
                Val::Percent(20.),
                Val::Percent(5.),
            );
            spawn_button(
                parent,
                MenuItemType::LevelEditor,
                menu_font.clone(),
                "Level Editor",
                Val::Percent(20.),
                Val::Percent(5.),
            );
            spawn_button(
                parent,
                MenuItemType::Exit,
                menu_font.clone(),
                "Exit",
                Val::Percent(20.),
                Val::Percent(5.),
            );
        });
}
