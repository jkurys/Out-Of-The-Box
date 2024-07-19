use bevy::prelude::*;
use itertools::Itertools;

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
            | GameObject::Empty,
    )
}

fn can_block_move(
    board: &ResMut<Board>,
    block: Block,
    dir: Direction,
    next_blocks: &mut Vec<Block>,
    visited_blocks: &mut Vec<Block>,
) -> bool {
    visited_blocks.push(block.clone());
    if board.is_block_empty(&block) {
        return false;
    }
    for &position in block.positions.iter() {
        if board.get_object_type(position) == GameObject::Empty {
            continue;
        }
        if !is_moveable(board.get_object_type(position)) {
            return false;
        }
        let (next_position, _next_map) =
            board.get_next_position_for_move(position, dir, board.get_current_map());
        let next_block = board.get_block(next_position);
        if board.is_block_empty(&next_block) 
            || board.get_object_type(next_position) == GameObject::Empty {
            continue;
        }
        let can_move = if visited_blocks.contains(&next_block) {
            true
        } else {
            can_block_move(board, next_block.clone(), dir, next_blocks, visited_blocks)
        };
        if next_block != block && !can_move {
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
    // TODO: pewnie brak sortowania robi czasem psucie kleju/blokow
    // na razie moze bez kleju tak o
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

        if board.is_block_empty(&next_block) {
            next_blocks.push(block.clone());
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
        let met_stationary_block = !is_position_in_blocks(all_blocks, next_position);

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
        .unique()
        .collect();
    positions_vec.sort_by(sort_positions(direction));
    for (position, _) in positions_vec {
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
    let mut visited_blocks = Vec::new();
    if !can_block_move(
        board,
        block,
        direction,
        &mut next_blocks,
        &mut visited_blocks,
    ) {
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
        block.clone(),
        all_blocks,
        direction,
        &mut next_blocks,
        &mut blocks_that_must_move,
    );
    let was_moved = can_block_move || !blocks_that_must_move.is_empty();
    perform_move(blocks_that_must_move, board, direction, writer);
    if can_block_move {
        perform_move(next_blocks, board, direction, writer);
    }
    return was_moved;
}
