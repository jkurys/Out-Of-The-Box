use bevy::prelude::*;

use crate::{
    board::Board,
    game::{
        game_objects::{Floor, GameObject},
        resources::BoardStates,
    },
    state::MoveState,
};

use super::events::{EnteredFloorEvent, TryMoveEvent};

pub fn try_move(
    mut reader: EventReader<TryMoveEvent>,
    mut writer: EventWriter<EnteredFloorEvent>,
    mut board: ResMut<Board>,
    mut board_states: ResMut<BoardStates>,
    mut app_state: ResMut<NextState<MoveState>>,
) {
    let mut was_moved = false;
    let mut was_map_saved = false;
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
    let mut moved_positions = Vec::new();
    for &TryMoveEvent {
        mut position,
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
            let mut positions_to_move = Vec::new();
            let (mut next_position, mut next_map) =
                board.get_next_position_for_move(position, direction, board.get_current_map());
            positions_to_move.push((position, board.get_current_map()));
            //we iterate to see if there is an empty space after some boxes
            while !moved_positions.contains(&next_position)
                && !positions.contains(&next_position)
                && (board.get_object_from_map(next_position, next_map) == GameObject::Box
                    || board.get_object_from_map(next_position, next_map) == GameObject::Player)
            {
                position = next_position;
                positions_to_move.push((position, next_map));
                (next_position, next_map) =
                    board.get_next_position_for_move(next_position, direction, next_map);
            }
            positions_to_move.reverse(); //we want to move the last box as first, so that they don't overlap
            let object_blocking = board.get_object_from_map(next_position, next_map);
            if object_blocking == GameObject::Empty {
                if !was_map_saved {
                    board_states.boards.push(board.clone());
                    was_map_saved = true;
                }
                for (position, map) in positions_to_move {
                    board.move_object(position, direction, map);
                    let next_position = position.next_position(direction);
                    writer.send(EnteredFloorEvent {
                        floor: board.get_floor_from_map(next_position, map),
                        position: next_position,
                        direction,
                        object: board.get_object_from_map(next_position, map),
                    });
                    moved_positions.push(position);
                    was_moved = true;
                }
            }
        } else {
            let mut position_to_move = (position, board.get_current_map());
            let (mut next_position, mut next_map) =
                board.get_next_position_for_move(position, direction, board.get_current_map());
            //we iterate to see if there is an empty space after some boxes
            let mut can_move = true;
            while !moved_positions.contains(&next_position)
                && board.get_object_from_map(next_position, next_map) == GameObject::Box
            {
                if board.get_floor_from_map(next_position, next_map) != Floor::Ice {
                    can_move = false;
                }
                position = next_position;
                position_to_move = (position, next_map);
                (next_position, next_map) =
                    board.get_next_position_for_move(next_position, direction, next_map);
            }
            let object_blocking = board.get_object_from_map(next_position, next_map);
            if can_move
                && (moved_positions.contains(&next_position)
                    || object_blocking == GameObject::Empty
                    || object_blocking == GameObject::Player)
            {
                let map = position_to_move.1;
                board.move_object(position, direction, map);
                let next_position = position.next_position(direction);
                writer.send(EnteredFloorEvent {
                    floor: board.get_floor_from_map(next_position, map),
                    position: next_position,
                    direction,
                    object: board.get_object_from_map(next_position, map),
                });
                was_moved = true;
                moved_positions.push(position_to_move.0);
            }
        }
        if let Some((object, position)) = insert_after {
            board.insert_object(position, object);
        }
    }
    if was_moved {
        app_state.set(MoveState::Animation);
    } else {
        app_state.set(MoveState::Static);
    }
}
