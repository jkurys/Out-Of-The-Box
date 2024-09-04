use bevy::prelude::*;
use bevy::color::palettes::css::{WHITE, GRAY};
use crate::state::DisplayState;

use super::setup::ButtonType;

pub fn handle_click(
    mut query: Query<(&Interaction, &ButtonType, &mut BackgroundColor), With<ButtonType>>,
    mut app_state: ResMut<NextState<DisplayState>>,
) {
    for (&interaction, &button_type, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => match button_type {
                ButtonType::Back => {
                    app_state.set(DisplayState::MainMenu);
                }
                ButtonType::Edit => {
                    app_state.set(DisplayState::LevelEditorLevelSelect);
                }
                ButtonType::New => {
                    app_state.set(DisplayState::LevelEditorInput);
                }
            },
            Interaction::Hovered => {
                *color = GRAY.into();
                // *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = WHITE.into();
                // *color = Color::WHITE.into();
            }
        }
    }
}
