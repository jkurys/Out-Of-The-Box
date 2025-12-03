use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

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

#[derive(Default)]
pub struct InputTextData {
    width: u32,
    height: u32,
    is_width_provided: bool,
}

pub fn handle_level_editor_input(
    mut char_reader: EventReader<KeyboardInput>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut local_data: Local<InputTextData>,
    mut app_state: ResMut<NextState<DisplayState>>,
    mut board_size: ResMut<BoardSize>,
    mut change_prompt: Query<&mut Text, QueryForInputTextConstraint>,
    mut change_number: Query<&mut Text, QueryForInputNumberConstraint>,
) {
    let InputTextData {
        width,
        height,
        is_width_provided,
    } = *local_data;
    for ev in char_reader.read() {
        if !ev.state.is_pressed() {
            continue;
        }
        match &ev.logical_key {
            Key::Character(character) => {
                let c = character.chars().last().unwrap();
                if c.is_ascii_digit() && !is_width_provided {
                    local_data.width = width * 10 + c.to_digit(10).unwrap();
                    if width > MAX_WIDTH {
                        local_data.width = 0;
                    }
                    let mut text = change_number.single_mut().expect("Number text not found");
                    *text = Text::new(local_data.width.to_string());
                } else if c.is_ascii_digit() {
                    local_data.height = height * 10 + c.to_digit(10).unwrap();
                    if height > MAX_HEIGHT {
                        local_data.height = 0;
                    }
                    let mut text = change_number.single_mut().expect("Number text not found");
                    *text = Text::new(local_data.height.to_string());
                }
            }
            _ => (),
        };
    }
    if input.just_pressed(KeyCode::Enter) && !is_width_provided {
        local_data.is_width_provided = true;
        input.reset(KeyCode::Enter);
        let mut text = change_prompt.single_mut().expect("Asking text not found");
        *text = Text::new("Please provide the level height".to_string());
        let mut text2 = change_number.single_mut().expect("Number text not found");
        *text2 = Text::new(0.to_string());
    }
    if input.just_pressed(KeyCode::Enter) && is_width_provided {
        *board_size = BoardSize { width, height };
        app_state.set(DisplayState::LevelEditorBoard);
        *local_data = InputTextData {
            width: 0,
            height: 0,
            is_width_provided: false,
        };
    }
}
