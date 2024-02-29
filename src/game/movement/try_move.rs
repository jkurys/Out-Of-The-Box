use bevy::prelude::*;

use crate::{
    board::Board,
    game::{game_objects::GameObject, resources::BoardStates},
    state::MoveState,
};

use super::{
    // events::{EnteredFloorEvent, TryMoveEvent},
    utils::{move_strong, move_weak}, resources::{MoveData, PushAttempt},
};

pub fn try_move(
    // mut reader: EventReader<TryMoveEvent>,
    // mut writer: EventWriter<EnteredFloorEvent>,
    mut move_data: ResMut<MoveData>,
    mut board: ResMut<Board>,
    mut board_states: ResMut<BoardStates>,
    mut app_state: ResMut<NextState<MoveState>>,
) {
    let mut was_map_saved = false;
    let mut was_moved = false;
    let mut events = Vec::new();
    let mut ice_events = Vec::new();
    let mut all_blocks = Vec::new();
    let push_clone = move_data.push_atempts.clone();
    for attempt in push_clone.iter() {
        if attempt.is_weak {
            ice_events.push(attempt);
            all_blocks.push(attempt.block.clone());
        } else {
            events.push(attempt);
        }
    }
    // trzeba zrobic zeby ruchy rzuwiowe dzialy sie po ruchu ktory nacisnal guzik + jakis priorytet
    // events.sort_by(|event1, event2| event1.block.cmp_to_other(&event2.block, event1.direction));
    for PushAttempt {
        block,
        direction,
        is_weak: _,
        insert_after,
    } in events.iter()
    {
        let can_block_move = move_strong(&mut board, block.clone(), *direction, &mut move_data);
        was_moved = was_moved || can_block_move;
        if !was_map_saved {
            board_states.boards.push(board.clone());
            was_map_saved = true;
        }
        if let Some((object, position)) = insert_after {
            if board.get_object_type(*position) == GameObject::Empty {
                board.insert_object(*position, *object);
            }
        }
    }
    for PushAttempt {
        block,
        direction,
        insert_after: _,
        is_weak: _,
    } in ice_events.iter()
    {
        let can_block_move = move_weak(
            &mut board,
            block.clone(),
            &all_blocks,
            *direction,
            &mut move_data,
        );
        was_moved = was_moved || can_block_move;
        // no insert_after_implemented here
    }
    move_data.push_atempts.clear();
    if was_moved {
        app_state.set(MoveState::Animation);
    } else {
        app_state.set(MoveState::Static);
    }
}
