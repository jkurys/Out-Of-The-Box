use bevy::prelude::*;

use crate::{
    consts::{MAX_HEIGHT, MAX_WIDTH},
    menu::level_editor::resources::BoardSize,
    state::DisplayState,
};
type QueryForInputTextConstraint = (
    With<LevelEditorStartingPrompt>,
    Without<LevelEditorInputNumber>,
);

type QueryForInputNumberConstraint = (
    With<LevelEditorInputNumber>,
    Without<LevelEditorStartingPrompt>,
);
use super::{LevelEditorInputNumber, LevelEditorStartingPrompt};

pub fn handle_level_editor_input(
    mut char_reader: EventReader<ReceivedCharacter>,
    mut input: ResMut<Input<KeyCode>>,
    mut local_data: Local<(u32, u32, bool)>,
    mut app_state: ResMut<NextState<DisplayState>>,
    mut board_size: ResMut<BoardSize>,
    mut change_prompt: Query<(&mut Text, QueryForInputTextConstraint)>,
    mut change_number: Query<(&mut Text, QueryForInputNumberConstraint)>,
) {
    let (width, height, is_width_provided) = *local_data;
    for ev in char_reader.iter() {
        if ev.char.is_ascii_digit() && !is_width_provided {
            local_data.0 = width * 10 + ev.char.to_digit(10).unwrap();
            if width > MAX_WIDTH {
                local_data.0 = 0;
            }
            let (mut text, _) = change_number.single_mut();
            text.sections[0].value = local_data.0.to_string();
        } else if ev.char.is_ascii_digit() {
            local_data.1 = height * 10 + ev.char.to_digit(10).unwrap();
            if height > MAX_HEIGHT {
                local_data.1 = 0;
            }
            let (mut text, _) = change_number.single_mut();
            text.sections[0].value = local_data.1.to_string();
        }
    }
    if input.just_pressed(KeyCode::Return) && !is_width_provided {
        local_data.2 = true;
        input.reset(KeyCode::Return);
        let (mut text, _) = change_prompt.single_mut();
        text.sections[0].value = "Please provide the level height".to_string();
        let (mut text2, _) = change_number.single_mut();
        text2.sections[0].value = 0.to_string();
    }
    if input.just_pressed(KeyCode::Return) && is_width_provided {
        *board_size = BoardSize { width, height };
        app_state.set(DisplayState::LevelEditorBoard);
        *local_data = (0, 0, false);
    }
}
