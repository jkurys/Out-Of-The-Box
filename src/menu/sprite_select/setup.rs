use bevy::prelude::*;

use crate::{
    consts::{MAIN_MENU_FONT, PLAYER_TEXTURES},
    menu::spawn_button,
};

use super::handle_click::{SpriteSelectItem, SpriteSelectItemType};

pub fn setup_sprite_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .insert(SpriteSelectItem)
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select Sprite"),
                TextFont {
                    font: menu_font.clone(),
                    font_size: 50.,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            spawn_button(
                parent,
                SpriteSelectItemType::Back,
                menu_font.clone(),
                "back",
                Val::Percent(20.),
                Val::Percent(10.),
            );
            parent.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ImageNode::new(asset_server.load(PLAYER_TEXTURES[0])),
            ));
            spawn_button(
                parent,
                SpriteSelectItemType::Select(0),
                menu_font.clone(),
                "select 0",
                Val::Percent(20.),
                Val::Percent(10.),
            );
            parent.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ImageNode::new(asset_server.load(PLAYER_TEXTURES[1])),
            ));
            spawn_button(
                parent,
                SpriteSelectItemType::Select(1),
                menu_font.clone(),
                "select 1",
                Val::Percent(20.),
                Val::Percent(10.),
            );
            parent.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ImageNode::new(asset_server.load(PLAYER_TEXTURES[2])),
            ));
            spawn_button(
                parent,
                SpriteSelectItemType::Select(2),
                menu_font.clone(),
                "select 2",
                Val::Percent(20.),
                Val::Percent(10.),
            );
        });
}
