use bevy::prelude::*;

use crate::{board::GameData, state::MoveState};

use super::{
    events::{EnteredFloorMessage, TryMoveMessage},
    resources::FireAnimation,
    strong::move_strong,
    weak::move_weak,
};

pub fn try_move(
    mut reader: MessageReader<TryMoveMessage>,
    mut writer: MessageWriter<EnteredFloorMessage>,
    mut game_data: ResMut<GameData>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut fire_animation: ResMut<FireAnimation>,
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

    for TryMoveMessage {
        block,
        direction,
        is_weak: _,
        position,
        is_long,
    } in events.iter()
    {
        if *is_long {
            let mut can_block_move =
                move_strong(&mut game_data, block.clone(), *direction, &mut writer);
            was_moved = was_moved || can_block_move;
            let mut next_position = position.next_position(*direction);
            let mut i = 0;
            while can_block_move && i < 20 {
                i += 1;
                let block = game_data.board.get_block(next_position);
                can_block_move =
                    move_strong(&mut game_data, block.clone(), *direction, &mut writer);
                next_position = next_position.next_position(*direction);
            }
        } else {
            let can_block_move =
                move_strong(&mut game_data, block.clone(), *direction, &mut writer);
            was_moved = was_moved || can_block_move;
        }
    }
    for TryMoveMessage {
        block,
        direction,
        is_weak: _,
        position: _,
        is_long: _,
    } in ice_events.iter()
    {
        let can_block_move = move_weak(
            &mut game_data,
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
