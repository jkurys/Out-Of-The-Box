use bevy::prelude::*;
use itertools::Itertools;

use crate::board::Board;
use crate::game::game_objects::{Block, Direction, Position};
use crate::state::MoveState;

use super::events::TryMoveEvent;

pub fn handle_keypress(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    board: ResMut<Board>,
    mut writer: EventWriter<TryMoveEvent>,
    mut app_state: ResMut<NextState<MoveState>>,
) {
    let direction = if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        Direction::Up
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        Direction::Down
    } else if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        Direction::Right
    } else {
        return;
    };
    let mut positions = board.get_player_positions();
    
    positions.sort_by(|&pos1, &pos2| match direction {
        Direction::Down => pos1.y.cmp(&pos2.y),
        Direction::Left => pos1.x.cmp(&pos2.x),
        Direction::Right => pos2.x.cmp(&pos1.x),
        Direction::Up => pos2.y.cmp(&pos1.y),
    });
    let blocks: Vec<(Block, Position)> = positions
        .into_iter()
        .map(|p| (board.get_block(p), p))
        .unique()
        .collect();
    
    for (block, position) in blocks {
        writer.send(TryMoveEvent {
            position,
            block,
            direction,
            is_weak: false,
        });
    }
    
    app_state.set(MoveState::Calculating);
}
