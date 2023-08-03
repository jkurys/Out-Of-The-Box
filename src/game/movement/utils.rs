use bevy::prelude::*;

use crate::{
    board::Board,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
};

use super::{events::EnteredFloorEvent, sort_positions::sort_positions};

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

fn is_block_empty(block: &Block, board: &ResMut<Board>) -> bool {
    for &position in block.positions.iter() {
        if board.get_object_type(position) != GameObject::Empty {
            return false;
        }
    }
    true
}

fn can_block_move(
    board: &ResMut<Board>,
    block: Block,
    dir: Direction,
    next_blocks: &mut Vec<Block>,
) -> bool {
    for &position in block.positions.iter() {
        if !is_moveable(board.get_object_type(position)) {
            return false;
        }
        let (next_position, _next_map) =
            board.get_next_position_for_move(position, dir, board.get_current_map());
        let next_block = board.get_block(next_position);
        if is_block_empty(&next_block, board) {
            continue;
        }
        if next_block != block && !can_block_move(board, next_block.clone(), dir, next_blocks) {
            return false;
        }
        if next_block != block {
            next_blocks.push(next_block)
        }
    }
    return true;
}

fn is_position_in_blocks(blocks: &Vec<Block>, position: Position) -> bool {
    for block in blocks {
        if block.contains_position(position) {
            return true;
        }
    }
    false
}

fn can_block_move_weak(
    board: &ResMut<Board>,
    block: Block,
    all_blocks: &Vec<Block>,
    dir: Direction,
    next_blocks: &mut Vec<Block>,
    blocks_that_must_move: &mut Vec<Block>,
) -> bool {
    for &position in block.positions.iter() {
        let can_current_block_move_somehow = is_moveable(board.get_object_type(position))
            && board.get_floor_type(position) == Floor::Ice;
        if !can_current_block_move_somehow {
            return false;
        }

        let mut next_block = block.clone();
        let mut next_position = position;
        let mut curr_position;
        while next_block == block {
            curr_position = next_position;
            next_position = board
                .get_next_position_for_move(curr_position, dir, board.get_current_map())
                .0;
            next_block = board.get_block(next_position);
        }

        if is_block_empty(&next_block, board) {
            blocks_that_must_move.push(block.clone());
            continue;
        }
        if !can_block_move_weak(
            board,
            next_block.clone(),
            all_blocks,
            dir,
            next_blocks,
            blocks_that_must_move,
        ) {
            return false;
        }
        let met_stationary_block = is_position_in_blocks(all_blocks, next_position);

        if met_stationary_block {
            blocks_that_must_move.push(next_block.clone());
            return false;
        }
        if is_position_in_blocks(all_blocks, position) && next_block == block {
            next_blocks.push(block.clone());
        }
    }
    return true;
}

fn perform_move(
    blocks: Vec<Block>,
    board: &mut ResMut<Board>,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
) {
    let mut positions_vec: Vec<(Position, usize)> = blocks
        .iter()
        .map(|block| block.positions.clone())
        .flatten()
        .map(|p| (p, 0))
        .collect();
    positions_vec.sort_by(sort_positions(direction));
    for (position, _) in positions_vec {
        if board.get_object_type(position) == GameObject::Empty {
            continue;
        }
        let map = board.get_current_map();
        board.move_object(position, direction, map);
        let next_position = board.get_next_position_for_move(position, direction, map).0;
        writer.send(EnteredFloorEvent {
            floor: board.get_floor_type(next_position),
            position: next_position,
            object: board.get_object_type(next_position),
            direction,
        });
    }
}

pub fn move_strong(
    board: &mut ResMut<Board>,
    block: Block,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
) -> bool {
    let mut next_blocks = vec![block.clone()];
    if !can_block_move(board, block, direction, &mut next_blocks) {
        return false;
    }
    perform_move(next_blocks, board, direction, writer);
    return true;
}

pub fn move_weak(
    board: &mut ResMut<Board>,
    block: Block,
    all_blocks: &Vec<Block>,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
) -> bool {
    let mut next_blocks = Vec::new();
    let mut blocks_that_must_move = Vec::new();
    let can_block_move = can_block_move_weak(
        board,
        block,
        all_blocks,
        direction,
        &mut next_blocks,
        &mut blocks_that_must_move,
    );
    perform_move(blocks_that_must_move, board, direction, writer);
    if !can_block_move {
        return false;
    }
    perform_move(next_blocks, board, direction, writer);
    return true;
}
// pub fn calc_positions_to_move_strong(
//     board: &mut ResMut<Board>,
//     mut position_before_move: Position,
//     direction: Direction,
//     next_move_positions: &mut Vec<(Position, usize)>,
//     moved_positions: &mut Vec<Position>,
// ) -> (Position, usize) {
//     let (mut next_position, mut next_map) =
//         board.get_next_position_for_move(position_before_move, direction, board.get_current_map());
//     next_move_positions.push((position_before_move, board.get_current_map()));
//     while !moved_positions.contains(&next_position)
//         && (is_moveable(board.get_object_from_map(next_position, next_map)))
//     {
//         if let GameObject::TurtleHead {
//             direction: turtle_head_direction,
//             color: _,
//         } = board.get_object_type(next_position)
//         {
//             if turtle_head_direction != direction && turtle_head_direction.opposite() != direction {
//                 let body_position = next_position.prev_position(turtle_head_direction);
//                 calc_positions_to_move_strong(
//                     board,
//                     body_position,
//                     direction,
//                     next_move_positions,
//                     moved_positions,
//                 );
//             }
//         }
//         if let GameObject::Turtle {
//             direction: turtle_direction,
//             color: _,
//         } = board.get_object_type(next_position)
//         {
//             if turtle_direction != direction && turtle_direction.opposite() != direction {
//                 let head_position = next_position.next_position(turtle_direction);
//                 if let GameObject::TurtleHead {
//                     direction: _,
//                     color: _,
//                 } = board.get_object_type(head_position)
//                 {
//                     calc_positions_to_move_strong(
//                         board,
//                         head_position,
//                         direction,
//                         next_move_positions,
//                         moved_positions,
//                     );
//                 }
//             }
//         }
//         position_before_move = next_position;
//         next_move_positions.push((position_before_move, next_map));
//         (next_position, next_map) =
//             board.get_next_position_for_move(next_position, direction, next_map);
//     }
//     (next_position, next_map)
// }
//
// pub fn move_strong(
//     board: &mut ResMut<Board>,
//     block: Block,
//     direction: Direction,
//     moved_positions: &mut Vec<Position>,
//     was_moved: &mut bool,
//     writer: &mut EventWriter<EnteredFloorEvent>,
// ) {
//     let mut next_move_positions = Vec::new();
//     let (next_position, next_map) = calc_positions_to_move_strong(
//         board,
//         position_before_move,
//         direction,
//         &mut next_move_positions,
//         moved_positions,
//     );
//     let object_blocking = board.get_object_from_map(next_position, next_map);
//     if object_blocking == GameObject::Empty {
//         next_move_positions.sort_by(sort_positions(direction));
//         next_move_positions.dedup();
//         for (position, map) in next_move_positions {
//             board.move_object(position, direction, map);
//             let next_position = position.next_position(direction);
//             writer.send(EnteredFloorEvent {
//                 floor: board.get_floor_from_map(next_position, map),
//                 position: next_position,
//                 direction,
//                 object: board.get_object_from_map(next_position, map),
//             });
//             moved_positions.push(position);
//             *was_moved = true;
//         }
//     }
// }
//
// pub fn move_weak(
//     board: &mut ResMut<Board>,
//     block: Block,
//     direction: Direction,
//     moved_positions: &mut Vec<Position>,
//     was_moved: &mut bool,
//     writer: &mut EventWriter<EnteredFloorEvent>,
// ) {
//     let mut can_block_move = false;
//     // for position in block.positions.iter() {
//     let mut position_to_move = (position, board.get_current_map());
//     let (mut next_position, mut next_map) =
//         board.get_next_position_for_move(position, direction, board.get_current_map());
//     // if let GameObject::TurtleHead {
//     //     direction: dir,
//     //     color: _,
//     // } = board.get_object_type(next_position)
//     // {
//     //     let body_position = next_position.prev_position(dir);
//     //     move_weak(
//     //         board,
//     //         body_position,
//     //         direction,
//     //         moved_positions,
//     //         was_moved,
//     //         writer,
//     //     );
//     // }
//     // if let GameObject::Turtle {
//     //     direction: dir,
//     //     color: _,
//     // } = board.get_object_type(next_position)
//     // {
//     //     let head_position = next_position.next_position(dir);
//     //     move_weak(
//     //         board,
//     //         head_position,
//     //         direction,
//     //         moved_positions,
//     //         was_moved,
//     //         writer,
//     //     );
//     // }
//     let mut can_move = true;
//     while !moved_positions.contains(&next_position)
//         && is_moveable(board.get_object_from_map(next_position, next_map))
//     {
//         if board.get_floor_from_map(next_position, next_map) != Floor::Ice {
//             can_move = false;
//         }
//         position = next_position;
//         position_to_move = (position, next_map);
//         (next_position, next_map) =
//             board.get_next_position_for_move(next_position, direction, next_map);
//     }
//     let object_blocking = board.get_object_from_map(next_position, next_map);
//     if can_move
//         && (moved_positions.contains(&next_position) || object_blocking == GameObject::Empty)
//     {
//         let map = position_to_move.1;
//         board.move_object(position, direction, map);
//         let next_position = position.next_position(direction);
//         writer.send(EnteredFloorEvent {
//             floor: board.get_floor_from_map(next_position, map),
//             position: next_position,
//             direction,
//             object: board.get_object_from_map(next_position, map),
//         });
//         *was_moved = true;
//         moved_positions.push(position_to_move.0);
//     }
// }
