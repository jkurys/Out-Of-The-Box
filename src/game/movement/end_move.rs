use bevy::prelude::*;

use crate::state::MoveState;

use super::events::TryMoveEvent;

pub fn end_move(reader: EventReader<TryMoveEvent>, mut app_state: ResMut<NextState<MoveState>>) {
    if reader.is_empty() {
        app_state.set(MoveState::Static);
    } else {
        app_state.set(MoveState::Calculating);
    }
}
