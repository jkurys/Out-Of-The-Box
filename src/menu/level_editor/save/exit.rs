use std::{fs::File, io::Write};

use bevy::prelude::*;

use crate::{board::GameData, resources::StateStack, state::DisplayState};

use super::events::FileSavedEvent;

pub fn save_board_to_file(
    mut game_data: ResMut<GameData>,
    mut reader: EventReader<FileSavedEvent>,
    mut app_state: ResMut<NextState<DisplayState>>,
    mut state_stack: ResMut<StateStack>,
) {
    let mut file_name = "".to_string();
    for ev in reader.read() {
        file_name = ev.0.clone();
    }
    if file_name == *"" {
        return;
    }
    let mut file = File::create(format!("assets/maps/{}.txt", file_name)).unwrap();
    game_data.entity_storage.clear();
    let file_prelude = serde_json::to_string_pretty(&game_data.board);
    if let Ok(str) = file_prelude {
        let buf = str.chars().map(|c| c as u8).collect::<Vec<_>>();
        file.write_all(&buf[..]).unwrap();
    }
    app_state.set(state_stack.0.pop().unwrap_or(DisplayState::MainMenu));
}

pub fn clear_board(mut game_data: ResMut<GameData>) {
    game_data.board.clear();
}
