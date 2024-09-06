use bevy::prelude::*;

use crate::{state::MoveState, game::resources::BoardStates};

use super::{events::TryMoveEvent, BoardPreMove};

pub fn end_move(
    reader: EventReader<TryMoveEvent>, 
    mut app_state: ResMut<NextState<MoveState>>,
    mut boards: ResMut<BoardStates>,
    board_before_move: Res<BoardPreMove>,
) {
    if reader.is_empty() {
        app_state.set(MoveState::Static);
    } else {
        if boards.boards.len() == 0
            || boards.boards[boards.boards.len() - 1] != board_before_move.clone().0 {
            boards.boards.push(board_before_move.clone().0);
        }
        app_state.set(MoveState::Calculating);
    }
}
