use std::cmp::Ordering;

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
    matches!(
        obj,
        GameObject::Box
            | GameObject::Player
            | GameObject::Turtle {
                direction: _,
                color: _,
            }
            | GameObject::TurtleHead {
                direction: _,
                color: _,
            }
    )
}

pub fn calc_positions_to_move_strong(
    board: &mut ResMut<Board>,
    mut position: Position,
    direction: Direction,
    positions_to_move: &mut Vec<(Position, usize)>,
    moved_positions: &mut Vec<Position>,
    positions: &Vec<Position>,
) -> (Position, usize) {
    let (mut next_position, mut next_map) =
        board.get_next_position_for_move(position, direction, board.get_current_map());
    positions_to_move.push((position, board.get_current_map()));
    while !moved_positions.contains(&next_position)
        && !positions.contains(&next_position)
        && (is_moveable(board.get_object_from_map(next_position, next_map)))
    {
        // let mut new_positions_to_move = Vec::new();
        if let GameObject::TurtleHead {
            direction: dir,
            color: _,
        } = board.get_object_type(next_position)
        {
            if dir != direction && dir.opposite() != direction {
                let body_position = next_position.prev_position(dir);
                calc_positions_to_move_strong(
                    board,
                    body_position,
                    direction,
                    positions_to_move,
                    moved_positions,
                    positions,
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
                    calc_positions_to_move_strong(
                        board,
                        head_position,
                        direction,
                        positions_to_move,
                        moved_positions,
                        positions,
                    );
                }
            }
        }
        position = next_position;
        positions_to_move.push((position, next_map));
        (next_position, next_map) =
            board.get_next_position_for_move(next_position, direction, next_map);
    }
    (next_position, next_map)
}

type SortFn = dyn FnMut(&(Position, usize), &(Position, usize)) -> Ordering;

pub fn sort_positions(dir: Direction) -> Box<SortFn> {
    Box::new(
        move |&(pos1, _): &(Position, usize), &(pos2, _): &(Position, usize)| match dir {
            Direction::Down => {
                if pos1.y != pos2.y {
                    pos1.y.cmp(&pos2.y)
                } else {
                    pos1.x.cmp(&pos2.x)
                }
            }
            Direction::Left => {
                if pos1.x != pos2.x {
                    pos1.x.cmp(&pos2.x)
                } else {
                    pos1.y.cmp(&pos2.y)
                }
            }
            Direction::Right => {
                if pos1.x != pos2.x {
                    pos2.x.cmp(&pos1.x)
                } else {
                    pos2.y.cmp(&pos1.y)
                }
            }
            Direction::Up => {
                if pos1.y != pos2.y {
                    pos2.y.cmp(&pos1.y)
                } else {
                    pos2.x.cmp(&pos1.x)
                }
            }
        },
    )
}

pub fn move_strong(
    board: &mut ResMut<Board>,
    position: Position,
    direction: Direction,
    moved_positions: &mut Vec<Position>,
    positions: &Vec<Position>,
    was_map_saved: &mut bool,
    was_moved: &mut bool,
    writer: &mut EventWriter<EnteredFloorEvent>,
    board_states: &mut ResMut<BoardStates>,
) {
    let mut positions_to_move = Vec::new();
    let (next_position, next_map) = calc_positions_to_move_strong(
        board,
        position,
        direction,
        &mut positions_to_move,
        moved_positions,
        positions,
    );
    let object_blocking = board.get_object_from_map(next_position, next_map);
    if object_blocking == GameObject::Empty {
        if !*was_map_saved {
            board_states.boards.push(board.clone());
            *was_map_saved = true;
        }
        positions_to_move.sort_by(sort_positions(direction));
        positions_to_move.dedup();
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
    board: &mut ResMut<Board>,
    mut position: Position,
    direction: Direction,
    moved_positions: &mut Vec<Position>,
    was_moved: &mut bool,
    writer: &mut EventWriter<EnteredFloorEvent>,
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
            board,
            body_position,
            direction,
            moved_positions,
            was_moved,
            writer,
        );
    }
    if let GameObject::Turtle {
        direction: dir,
        color: _,
    } = board.get_object_type(next_position)
    {
        let head_position = next_position.next_position(dir);
        move_weak(
            board,
            head_position,
            direction,
            moved_positions,
            was_moved,
            writer,
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
