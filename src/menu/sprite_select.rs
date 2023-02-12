use super::spawn_button;
use crate::{
    consts::{MAIN_MENU_FONT, PLAYER_TEXTURES, PLAYER_TEXTURE_SAVE},
    state::DisplayState,
};
use bevy::prelude::*;
use std::fs::File;
use std::io::Write;
#[derive(Component)]
pub struct SpriteSelectItem;

#[derive(Component)]
pub enum SpriteSelectItemType {
    Select(u8),
    Back,
}

pub fn setup_sprite_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility { is_visible: true },
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
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );
            spawn_button(
                parent,
                SpriteSelectItemType::Back,
                menu_font.clone(),
                "back",
            );
            parent.spawn(ImageBundle {
                image: UiImage(asset_server.load(PLAYER_TEXTURES[0])),
                style: Style {
                    size: Size {
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                    },
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
                image: UiImage(asset_server.load(PLAYER_TEXTURES[1])),
                style: Style {
                    size: Size {
                        height: Val::Px(100.0),
                        width: Val::Px(100.0),
                    },
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
                image: UiImage(asset_server.load(PLAYER_TEXTURES[2])),
                style: Style {
                    size: Size {
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                    },
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

pub fn handle_sprite_click(
    mut app_state: ResMut<State<DisplayState>>,
    mut query: Query<
        (
            &mut Interaction,
            &mut BackgroundColor,
            &mut SpriteSelectItemType,
        ),
        With<SpriteSelectItemType>,
    >,
) {
    query.for_each_mut(
        |(interaction, mut color, item)| match interaction.as_ref() {
            Interaction::Clicked => match item.as_ref() {
                SpriteSelectItemType::Back => {
                    app_state.pop().expect("Going back to main menu failed");
                }
                SpriteSelectItemType::Select(sprite_no) => {
                    let bytes = [*sprite_no];
                    let mut file = File::create(PLAYER_TEXTURE_SAVE).unwrap();
                    file.write_all(&bytes).unwrap();
                    app_state.pop().expect("Going back to main menu failed");
                }
            },
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
        },
    )
}
