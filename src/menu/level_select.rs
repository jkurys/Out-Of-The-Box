use bevy::prelude::*;

use std::fs::File;
use std::io::Read;

use crate::{
    consts::{LEVEL_AMOUNT, LEVEL_SAVE, MAIN_MENU_FONT, MAP_NAMES},
    resources::CurrentLevel,
    state::DisplayState,
};

use super::spawn_button;
#[derive(Component)]
pub struct LevelSelectItem;

#[derive(Component)]
pub enum LevelSelectItemType {
    Level(usize),
    Back,
}

pub fn setup_level_select(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(LevelSelectItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Level Select",
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
            for level_number in 0..LEVEL_AMOUNT {
                spawn_button(
                    parent,
                    LevelSelectItemType::Level(level_number + 1),
                    menu_font.clone(),
                    format!("Level {}", level_number + 1).as_str(),
                );
            }

            spawn_button(parent, LevelSelectItemType::Back, menu_font.clone(), "back");
        });
}

pub fn handle_level_click(
    mut app_state: ResMut<State<DisplayState>>,
    mut query: Query<
        (
            &mut Interaction,
            &mut BackgroundColor,
            &mut LevelSelectItemType,
        ),
        With<LevelSelectItemType>,
    >,
    mut current_level: ResMut<CurrentLevel>,
) {
    let file = File::open(LEVEL_SAVE);
    let mut buf = [0; LEVEL_AMOUNT];
    if let Ok(mut file) = file {
        file.read_exact(&mut buf).unwrap();
    }
    let bool_buf = buf.map(|value| value != 0);
    query.for_each_mut(
        |(interaction, mut color, item)| match interaction.as_ref() {
            Interaction::Clicked => match item.as_ref() {
                LevelSelectItemType::Level(number) => {
                    *current_level = CurrentLevel {
                        level_number: *number,
                        level_map_str: MAP_NAMES[*number - 1],
                    };
                    app_state
                        .push(DisplayState::Game)
                        .expect("Failed to load game");
                }
                LevelSelectItemType::Back => {
                    app_state.pop().expect("Going back to main menu failed");
                }
            },
            Interaction::Hovered => {
                *color = match item.as_ref() {
                    LevelSelectItemType::Level(level_no) => {
                        if bool_buf[*level_no - 1] {
                            BackgroundColor(Color::DARK_GREEN)
                        } else {
                            BackgroundColor(Color::GRAY)
                        }
                    }
                    _ => BackgroundColor(Color::GRAY),
                }
            }
            Interaction::None => {
                *color = match item.as_ref() {
                    LevelSelectItemType::Level(level_no) => {
                        if bool_buf[*level_no - 1] {
                            BackgroundColor(Color::GREEN)
                        } else {
                            BackgroundColor(Color::WHITE)
                        }
                    }
                    _ => BackgroundColor(Color::WHITE),
                }
            }
        },
    )
}
