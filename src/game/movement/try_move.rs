use bevy::prelude::*;

use crate::{
    board::Board,
    game::{game_objects::GameObject, resources::BoardStates},
    state::MoveState,
};

use super::{
    events::{EnteredFloorEvent, TryMoveEvent},
    utils::{move_strong, move_weak, MoveData},
};

pub fn try_move(
    mut reader: EventReader<TryMoveEvent>,
    mut writer: EventWriter<EnteredFloorEvent>,
    mut board: ResMut<Board>,
    mut board_states: ResMut<BoardStates>,
    mut app_state: ResMut<NextState<MoveState>>,
) {
    let mut move_data = MoveData {
        was_map_saved: false,
        was_moved: false,
    };
    let mut events = Vec::new();
    for event in reader.iter() {
        events.push(event);
    }
    events.sort_by(|event1, event2| {
        event1
            .position
            .cmp_to_other(&event2.position, event1.direction)
    });
    let mut moved_positions = Vec::new();
    for &TryMoveEvent {
        position,
        direction,
        is_weak,
        insert_after,
    } in events
    {
        if board.get_object_type(position) == GameObject::Empty {
            if let Some((object, position)) = insert_after {
                board.insert_object(position, object);
            }
            continue;
        }
        if !is_weak {
            move_strong(
                &mut board,
                position,
                direction,
                &mut moved_positions,
                &mut move_data,
                &mut writer,
                &mut board_states,
            );
        } else {
            move_weak(
                &mut board,
                position,
                direction,
                &mut moved_positions,
                &mut move_data,
                &mut writer,
            );
        }
        if let Some((object, position)) = insert_after {
            if board.get_object_type(position) == GameObject::Empty {
                board.insert_object(position, object);
            }
        }
    }
    if move_data.was_moved {
        app_state.set(MoveState::Animation);
    } else {
        app_state.set(MoveState::Static);
    }
}
