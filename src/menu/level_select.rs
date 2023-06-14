use bevy::prelude::*;

use std::fs::File;
use std::fs::read_dir;
use std::io::Read;

use crate::resources::StateStack;
use crate::{
    consts::{LEVEL_SAVE, MAIN_MENU_FONT},
    resources::CurrentLevel,
    state::DisplayState,
};

use super::resources::LevelNames;
use super::spawn_button;
#[derive(Component)]
pub struct LevelSelectItem;

#[derive(Component)]
pub enum LevelSelectItemType {
    Level(usize),
    Back,
}

pub fn setup_level_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_names: ResMut<LevelNames>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let paths = read_dir("./assets/maps").unwrap();
    let mut file_amount = 0;
    let mut file_paths = Vec::new();
    let mut first_name = "".to_string();
    for path in paths {
        let path_str = path.unwrap().path().file_name().unwrap().to_string_lossy().into_owned();
        if first_name == "".to_string() {
            first_name = path_str.clone();
        }
        file_paths.push(path_str);
        file_amount += 1;
    }
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
                .with_text_alignment(TextAlignment::Center),
            );
            for level_number in 0..file_amount {
                let level_name = &file_paths[level_number];
                spawn_button(
                    parent,
                    LevelSelectItemType::Level(level_number + 1),
                    menu_font.clone(),
                    &level_name[..&level_name.len() - 4],
                );
            }

            spawn_button(parent, LevelSelectItemType::Back, menu_font.clone(), "back");
        });
    level_names.0 = file_paths;
    *current_level = CurrentLevel {
        level_amount: file_amount,
        level_number: 0,
        level_map_string: first_name,
    };
}

pub fn handle_level_click(
    app_state: ResMut<State<DisplayState>>,
    mut next_state: ResMut<NextState<DisplayState>>,
    mut state_stack: ResMut<StateStack>,
    mut query: Query<
        (
            &mut Interaction,
            &mut BackgroundColor,
            &mut LevelSelectItemType,
        ),
        With<LevelSelectItemType>,
    >,
    mut current_level: ResMut<CurrentLevel>,
    level_names: Res<LevelNames>,
) {
    let file = File::open(LEVEL_SAVE);
    let level_amount = current_level.level_amount;
    let mut buf = vec![0; level_amount];
    if let Ok(mut file) = file {
        file.read_exact(&mut buf).unwrap();
    }
    let bool_buf: Vec<bool> = buf.iter().map(|&value| value != 0).collect();
    query.for_each_mut(
        |(interaction, mut color, item)| match interaction.as_ref() {
            Interaction::Clicked => match item.as_ref() {
                LevelSelectItemType::Level(number) => {
                    *current_level = CurrentLevel {
                        level_number: *number,
                        level_map_string: level_names.0[*number - 1].clone(),
                        level_amount,
                    };
                    state_stack.0.push(app_state.0);
                    next_state.set(DisplayState::Game);
                }
                LevelSelectItemType::Back => {
                    next_state.set(state_stack.0.pop().expect("Going back to main menu failed"));
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
