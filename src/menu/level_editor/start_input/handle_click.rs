use bevy::prelude::*;

use crate::{menu::level_editor::resources::BoardSize, state::DisplayState, consts::{MAX_WIDTH, MAX_HEIGHT}};

use super::{LevelEditorStartingPrompt, LevelEditorInputNumber};

pub fn handle_level_editor_input(
    mut char_reader: EventReader<ReceivedCharacter>,
    mut input: ResMut<Input<KeyCode>>,
    mut width: Local<u32>,
    mut height: Local<u32>,
    mut is_width_provided: Local<bool>,
    mut app_state: ResMut<NextState<DisplayState>>,
    mut board_size: ResMut<BoardSize>,
    mut change_prompt: Query<(&mut Text, (With<LevelEditorStartingPrompt>, Without<LevelEditorInputNumber>))>,
    mut change_number: Query<(&mut Text, (With<LevelEditorInputNumber>, Without<LevelEditorStartingPrompt>))>,
) {
    for ev in char_reader.iter() {
        if ev.char.is_ascii_digit() && !*is_width_provided {
            *width = *width * 10 + ev.char.to_digit(10).unwrap();
            if *width > MAX_WIDTH {
                *width = 0;
            }
            let (mut text, _) = change_number.single_mut();
            text.sections[0].value = width.to_string();
        } else if ev.char.is_ascii_digit() {
            *height = *height * 10 + ev.char.to_digit(10).unwrap();
            if *height > MAX_HEIGHT {
                *height = 0;
            }
            let (mut text, _) = change_number.single_mut();
            text.sections[0].value = height.to_string();
        }
    }
    if input.just_pressed(KeyCode::Return) && !*is_width_provided {
        *is_width_provided = true;
        input.reset(KeyCode::Return);
        let (mut text, _) = change_prompt.single_mut();
        text.sections[0].value = "Please provide the level height".to_string();
        let (mut text2, _) = change_number.single_mut();
        text2.sections[0].value = 0.to_string();
    }
    if input.just_pressed(KeyCode::Return) && *is_width_provided {
        *is_width_provided = false;
        *board_size = BoardSize { width: *width, height: *height };
        app_state.set(DisplayState::LevelEditorBoard);
        *height = 0;
        *width = 0;
    }
}
