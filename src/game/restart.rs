use super::resources::BoardStates;
use crate::board::GameData;
use bevy::prelude::*;

pub fn handle_restart(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut boards: ResMut<BoardStates>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if !boards.boards.is_empty() {
            game_data.board = boards
                .boards
                .drain(..)
                .next()
                .expect("Boards was not empty");
            boards.boards.clear();
        }
        keyboard_input.reset(KeyCode::KeyR);
    }
}

pub fn handle_undo(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut boards: ResMut<BoardStates>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyU) && !boards.boards.is_empty() {
        game_data.board = boards.boards.pop().expect("Could not get last move");
        keyboard_input.reset(KeyCode::KeyU);
    }
}
