use bevy::prelude::*;

use crate::{board::Board, game::resources::BoardStates, state::MoveState};

use super::{
    events::{EnteredFloorEvent, TryMoveEvent},
    utils::{move_strong, move_weak}, resources::FireAnimation,
};

pub fn try_move(
    mut reader: EventReader<TryMoveEvent>,
    mut writer: EventWriter<EnteredFloorEvent>,
    mut board: ResMut<Board>,
    // mut board_states: ResMut<BoardStates>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut fire_animation: ResMut<FireAnimation>,
) {
    // let mut was_map_saved = false;
    let mut was_moved = false;
    let mut events = Vec::new();
    let mut ice_events = Vec::new();
    let mut all_blocks = Vec::new();
    for event in reader.read() {
        if event.is_weak {
            ice_events.push(event);
            all_blocks.push(event.block.clone());
        } else {
            events.push(event);
        }
    }
    // TODO: trzeba zrobic zeby ruchy rzuwiowe dzialy sie po ruchu ktory nacisnal guzik + jakis priorytet

    // events.sort_by(|event1, event2| event1.block.cmp_to_other(&event2.block, event1.direction));
    for TryMoveEvent {
        block,
        direction,
        is_weak: _,
        position,
    } in events.iter()
    {
        let can_block_move = move_strong(&mut board, block.clone(), *position, *direction, &mut writer);
        was_moved = was_moved || can_block_move;
        // if !was_map_saved {
        //     board_states.boards.push(board.clone());
        //     was_map_saved = true;
        // }
    }
    for TryMoveEvent {
        block,
        direction,
        is_weak: _,
        position: _,
    } in ice_events.into_iter()
    {
        let can_block_move = move_weak(
            &mut board,
            block.clone(),
            &all_blocks,
            *direction,
            &mut writer,
        );
        was_moved = was_moved || can_block_move;
    }
    if was_moved || fire_animation.0 {
        app_state.set(MoveState::Animation);
    } else {
        app_state.set(MoveState::Static);
    }
    fire_animation.0 = false;
}
