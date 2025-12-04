use bevy::prelude::*;

use crate::{game::resources::BoardStates, state::MoveState};

use super::{events::TryMoveMessage, BoardPreMove};

pub fn end_move(
    reader: MessageReader<TryMoveMessage>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut boards: ResMut<BoardStates>,
    board_before_move: Res<BoardPreMove>,
) {
    if reader.is_empty() {
        app_state.set(MoveState::Static);
    } else {
        if boards.boards.len() == 0
            || boards.boards[boards.boards.len() - 1] != board_before_move.clone().0
        {
            boards.boards.push(board_before_move.clone().0);
        }
        app_state.set(MoveState::Calculating);
    }
}
