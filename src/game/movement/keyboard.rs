use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};
use crate::game::resources::BoardStates;
use crate::board::Board;
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
    let mut positions = board.get_player_positions();
    positions.sort_by(|&pos1, &pos2| {
        match direction {
            Direction::Down => {
                pos1.y.cmp(&pos2.y)
            }
            Direction::Left => {
                pos1.x.cmp(&pos2.x)
            }
            Direction::Right => {
                pos2.x.cmp(&pos1.x)
            }
            Direction::Up => {
                pos2.y.cmp(&pos1.y)
            }
        }
    });
    for mut position in positions {
        let mut positions_to_move = Vec::new();
        let (mut next_position, mut next_map) = board.get_next_position_for_move(position, direction, board.get_current_map());
        positions_to_move.push((position, board.get_current_map()));
        //we iterate to see if there is an empty space after some boxes
        while board.get_object_from_map(next_position, next_map) == GameObject::Box {
            position = next_position;
            positions_to_move.push((position, next_map));
            (next_position, next_map) = board.get_next_position_for_move(next_position, direction, next_map);
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
            }
            app_state.set(MoveState::Moving);
        }
    }
}
