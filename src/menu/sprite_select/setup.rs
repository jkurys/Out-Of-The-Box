use bevy::prelude::*;

use crate::{
    consts::{MAIN_MENU_FONT, PLAYER_TEXTURES},
    menu::spawn_button,
};

use super::handle_click::{SpriteSelectItem, SpriteSelectItemType};

pub fn setup_sprite_select(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(SpriteSelectItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Select Sprite",
                    TextStyle {
                        font: menu_font.clone(),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
            spawn_button(
                parent,
                SpriteSelectItemType::Back,
                menu_font.clone(),
                "back",
            );
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load(PLAYER_TEXTURES[0]),
                    ..default()
                },
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            });
            spawn_button(
                parent,
                SpriteSelectItemType::Select(0),
                menu_font.clone(),
                "select 0",
            );
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load(PLAYER_TEXTURES[1]),
                    ..default()
                },
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            });
            spawn_button(
                parent,
                SpriteSelectItemType::Select(1),
                menu_font.clone(),
                "select 1",
            );
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load(PLAYER_TEXTURES[2]),
                    ..default()
                },
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            });
            spawn_button(
                parent,
                SpriteSelectItemType::Select(2),
                menu_font.clone(),
                "select 2",
            );
        });
}
