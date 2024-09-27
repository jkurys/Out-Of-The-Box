use bevy::prelude::*;

use crate::{board::Board, state::MoveState};

use super::{
    events::{EnteredFloorEvent, TryMoveEvent},
    utils::{move_strong, move_weak}, resources::{FireAnimation, DisplayButton},
};

pub fn try_move(
    mut reader: EventReader<TryMoveEvent>,
    mut writer: EventWriter<EnteredFloorEvent>,
    mut board: ResMut<Board>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut fire_animation: ResMut<FireAnimation>,
    mut display_button: ResMut<DisplayButton>,
) {
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

    for TryMoveEvent {
        block,
        direction,
        is_weak: _,
        position,
        is_long,
    } in events.iter()
    {
        if *is_long {
            let mut can_block_move = move_strong(&mut board, block.clone(), *position, *direction, &mut writer, false, &mut display_button);
            let mut next_position = position.next_position(*direction);
            let mut i = 0;
            while can_block_move && i < 20 {
                i += 1;
                let block = board.get_block(next_position);
                can_block_move = move_strong(&mut board, block.clone(), next_position, *direction, &mut writer, false, &mut display_button);
                next_position = next_position.next_position(*direction);
            }
        } else {
            let can_block_move = move_strong(&mut board, block.clone(), *position, *direction, &mut writer, false, &mut display_button);
            was_moved = was_moved || can_block_move;
        }
    }
    for TryMoveEvent {
        block,
        direction,
        is_weak: _,
        position: _,
        is_long: _,
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
