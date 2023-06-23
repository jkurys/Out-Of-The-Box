use bevy::prelude::*;

use crate::{
    board::Board,
    game::{
        game_objects::{Direction, Floor, GameObject, Position},
        resources::BoardStates,
    },
};

use super::events::EnteredFloorEvent;

pub fn is_moveable(obj: GameObject) -> bool {
    match obj {
        GameObject::Box => true,
        GameObject::Player => true,
        GameObject::Turtle {
            direction: _,
            color: _,
        } => true,
        GameObject::TurtleHead {
            direction: _,
            color: _,
        } => true,
        _ => false,
    }
}

pub fn move_strong(
    mut board: &mut ResMut<Board>,
    mut position: Position,
    direction: Direction,
    mut moved_positions: &mut Vec<Position>,
    positions: &Vec<Position>,
    mut was_map_saved: &mut bool,
    mut was_moved: &mut bool,
    mut writer: &mut EventWriter<EnteredFloorEvent>,
    mut board_states: &mut ResMut<BoardStates>,
) {
    let mut positions_to_move = Vec::new();
    let (mut next_position, mut next_map) =
        board.get_next_position_for_move(position, direction, board.get_current_map());
    if let GameObject::TurtleHead {
        direction: dir,
        color: _,
    } = board.get_object_type(next_position)
    {
        if dir != direction && dir.opposite() != direction {
            let body_position = next_position.prev_position(dir);
            move_strong(
                &mut board,
                body_position,
                direction,
                &mut moved_positions,
                &positions,
                &mut was_map_saved,
                &mut was_moved,
                &mut writer,
                &mut board_states,
            );
        }
    }
    if let GameObject::Turtle {
        direction: dir,
        color: _,
    } = board.get_object_type(next_position)
    {
        if dir != direction && dir.opposite() != direction {
            let head_position = next_position.next_position(dir);
            if let GameObject::TurtleHead {
                direction: _,
                color: _,
            } = board.get_object_type(head_position)
            {
                move_strong(
                    &mut board,
                    head_position,
                    direction,
                    &mut moved_positions,
                    &positions,
                    &mut was_map_saved,
                    &mut was_moved,
                    &mut writer,
                    &mut board_states,
                );
            }
        }
    }
    positions_to_move.push((position, board.get_current_map()));
    while !moved_positions.contains(&next_position)
        && !positions.contains(&next_position)
        && (is_moveable(board.get_object_from_map(next_position, next_map)))
    {
        position = next_position;
        positions_to_move.push((position, next_map));
        (next_position, next_map) =
            board.get_next_position_for_move(next_position, direction, next_map);
    }
    positions_to_move.reverse();
    let object_blocking = board.get_object_from_map(next_position, next_map);
    if object_blocking == GameObject::Empty {
        if !*was_map_saved {
            board_states.boards.push(board.clone());
            *was_map_saved = true;
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
            *was_moved = true;
        }
    }
}

pub fn move_weak(
    mut board: &mut ResMut<Board>,
    mut position: Position,
    direction: Direction,
    mut moved_positions: &mut Vec<Position>,
    mut was_moved: &mut bool,
    mut writer: &mut EventWriter<EnteredFloorEvent>,
) {
    let mut position_to_move = (position, board.get_current_map());
    let (mut next_position, mut next_map) =
        board.get_next_position_for_move(position, direction, board.get_current_map());
    if let GameObject::TurtleHead {
        direction: dir,
        color: _,
    } = board.get_object_type(next_position)
    {
        let body_position = next_position.prev_position(dir);
        move_weak(
            &mut board,
            body_position,
            direction,
            &mut moved_positions,
            &mut was_moved,
            &mut writer,
        );
    }
    if let GameObject::Turtle {
        direction: dir,
        color: _,
    } = board.get_object_type(next_position)
    {
        let head_position = next_position.next_position(dir);
        move_weak(
            &mut board,
            head_position,
            direction,
            &mut moved_positions,
            &mut was_moved,
            &mut writer,
        );
    }
    let mut can_move = true;
    while !moved_positions.contains(&next_position)
        && is_moveable(board.get_object_from_map(next_position, next_map))
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
        && (moved_positions.contains(&next_position) || object_blocking == GameObject::Empty)
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
        *was_moved = true;
        moved_positions.push(position_to_move.0);
    }
}
