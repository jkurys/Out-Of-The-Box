use bevy::prelude::*;

use crate::state::MoveState;

// use super::events::TryMoveEvent;
use super::resources::MoveData;

pub fn end_move(
    // reader: EventReader<TryMoveEvent>,
    move_data: ResMut<MoveData>,
    mut app_state: ResMut<NextState<MoveState>>,
) {
    if move_data.push_atempts.is_empty() {
        app_state.set(MoveState::Static);
    } else {
        app_state.set(MoveState::Calculating);
    }
}
