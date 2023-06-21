use bevy::prelude::*;

use crate::{menu::resources::LevelNames, resources::CurrentLevel, state::DisplayState};

use super::setup::ButtonType;

pub fn handle_click(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonType), With<ButtonType>>,
    mut next_state: ResMut<NextState<DisplayState>>,
    level_names: Res<LevelNames>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (&interaction, mut color, &button_type) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => match button_type {
                ButtonType::Level(number) => {
                    *current_level = CurrentLevel {
                        level_number: number,
                        level_map_string: level_names.0[number - 1].clone(),
                        ..*current_level
                    };
                    next_state.set(DisplayState::LevelEditorBoard);
                }
                ButtonType::Back => {
                    next_state.set(DisplayState::LevelEditorSelect);
                }
            },
            Interaction::Hovered => {
                *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}
