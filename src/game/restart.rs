use super::resources::BoardStates;
use crate::board::Board;
use bevy::prelude::*;

//BUG: sometimes restart restarts to a non strarting position

pub fn handle_restart(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    // mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if !boards.boards.is_empty() {
            *board = boards.boards[0].clone();
            boards.boards.clear();
        }
        keyboard_input.reset(KeyCode::KeyR);
    }
}

pub fn handle_undo(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    // mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyU) && !boards.boards.is_empty() {
        *board = boards.boards.pop().expect("Could not get last move");
        keyboard_input.reset(KeyCode::KeyU);
    }
}
