use super::resources::BoardStates;
use crate::consts::INITIAL_MAP;
use crate::resources::Board;
// use crate::state::CurrentMap;
use bevy::prelude::*;

pub fn handle_restart(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    // mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        if !boards.boards.is_empty() {
            *board = boards.boards[0].clone();
            boards.boards.clear();
        }
        if board.get_current_map() != INITIAL_MAP {
            board.set_current_map(INITIAL_MAP);
        }
        keyboard_input.reset(KeyCode::R);
    }
}

pub fn handle_undo(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    // mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::U) && !boards.boards.is_empty() {
        let old_map = board.get_current_map();
        *board = boards.boards.pop().expect("Could not get last move");
        let new_map = board.get_current_map();
        if old_map != new_map {
            board.set_current_map(new_map);
        }
        keyboard_input.reset(KeyCode::U);
    }
}
