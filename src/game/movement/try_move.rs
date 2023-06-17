use bevy::prelude::*;

use crate::{game::{game_objects::GameObject, resources::BoardStates}, board::Board, state::MoveState};

use super::events::{TryMoveEvent, ExitedFloorEvent};

pub fn try_move(
    mut reader: EventReader<TryMoveEvent>,
    mut writer: EventWriter<ExitedFloorEvent>,
    board: Res<Board>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut board_states: ResMut<BoardStates>,
) {
    let mut was_moved = false;
    let mut events = Vec::new();
    let mut positions = Vec::new();
    for event in reader.iter() {
        positions.push(event.position);
        events.push(event);
    }
    events.sort_by(|event1, event2| {
        event1
            .position
            .cmp_to_other(&event2.position, event1.direction)
    });
    for &TryMoveEvent { mut position, direction } in events {
        let mut positions_to_move = Vec::new();
        let (mut next_position, mut next_map) =
            board.get_next_position_for_move(position, direction, board.get_current_map());
        positions_to_move.push((position, board.get_current_map()));
        //we iterate to see if there is an empty space after some boxes
        while board.get_object_from_map(next_position, next_map) == GameObject::Box {
            position = next_position;
            positions_to_move.push((position, next_map));
            (next_position, next_map) =
                board.get_next_position_for_move(next_position, direction, next_map);
        }
        positions_to_move.reverse(); //we want to move the last box as first, so that they don't overlap
        let object_blocking = board.get_object_from_map(next_position, next_map);
        if object_blocking == GameObject::Empty || object_blocking == GameObject::Player {
            board_states.boards.push(board.clone());
            for (position, map) in positions_to_move {
                writer.send(ExitedFloorEvent {
                    floor: board.get_floor_from_map(position, map),
                    position,
                    direction,
                    map,
                    object: board.get_object_from_map(position, map),
                });
                was_moved = true;
            }
        }
    }
    if was_moved {
        app_state.set(MoveState::Moving);
    } else {
        app_state.set(MoveState::Static);
    }
}
