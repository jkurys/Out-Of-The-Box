use std::{fs::File, io::Read};

use bevy::prelude::*;

use crate::{state::DisplayState, resources::{StateStack, CurrentLevel}, menu::resources::LevelNames, consts::LEVEL_SAVE};

use super::LevelSelectItemType;

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
                    next_state.set(state_stack.0.pop().unwrap_or(DisplayState::MainMenu));
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
