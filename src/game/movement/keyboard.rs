use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};
use crate::game::resources::BoardStates;
use crate::resources::Board;
use crate::state::MoveState;

use super::events::ExitedFloorEvent;

pub fn handle_keypress(
    keyboard_input: ResMut<Input<KeyCode>>,
    board: Res<Board>,
    mut writer: EventWriter<ExitedFloorEvent>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut board_states: ResMut<BoardStates>,
) {
    let direction = if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        Direction::Up
    } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        Direction::Down
    } else if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        Direction::Right
    } else {
        return;
    };
    let mut position = board.get_player_position();
    let mut positions = Vec::new();
    let (mut next_position, mut next_map) = board.get_next_position_for_move(position, direction, board.get_current_map());
    // let mut next_position = position.next_position(direction);
    positions.push((position, board.get_current_map()));
    //we iterate to see if there is an empty space after some boxes
    while board.get_object_from_map(next_position, next_map) == GameObject::Box {
        position = next_position;
        positions.push((position, next_map));
        (next_position, next_map) = board.get_next_position_for_move(next_position, direction, next_map);
        // next_position = next_position.next_position(direction);
    }
    positions.reverse(); //we want to move the last box as first, so that they don't overlap
    let object_blocking = board.get_object_from_map(next_position, next_map);
    if object_blocking == GameObject::Empty {
        board_states.boards.push(board.clone());
        for (position, map) in positions {
            writer.send(ExitedFloorEvent {
                floor: board.get_floor_from_map(position, map),
                position,
                direction,
                map,
                object: board.get_object_from_map(position, map),
            });
        }
        app_state.set(MoveState::Moving);
    }
}
