use super::resources::BoardStates;
use crate::consts::INITIAL_MAP;
use crate::resources::Board;
use crate::state::CurrentMap;
use bevy::prelude::*;

pub fn handle_restart(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        if !boards.boards.is_empty() {
            *board = boards.boards[0].clone();
            boards.boards.clear();
        }
        if current_map.current() != &CurrentMap(Some(INITIAL_MAP)) {
            current_map
                .set(CurrentMap(Some(INITIAL_MAP)))
                .expect("Could not restart");
        }
        keyboard_input.reset(KeyCode::R);
    }
}

pub fn handle_undo(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::U) && !boards.boards.is_empty() {
        *board = boards.boards.pop().expect("Could not get last move");
        let new_map = board.get_current_map();
        if let CurrentMap(Some(state_map)) = current_map.current() {
            if *state_map != new_map {
                current_map
                    .set(CurrentMap(Some(new_map)))
                    .expect("Could not undo map state");
            }
        }
        keyboard_input.reset(KeyCode::U);
    }
}
