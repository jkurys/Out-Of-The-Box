use bevy::prelude::*;

use crate::{
    board::Board,
    game::game_objects::{Block, Direction, Floor, GameObject},
};

use super::{events::EnteredFloorEvent, utils::{is_moveable, perform_move, is_position_in_blocks}};


fn can_block_move_weak(
    board: &ResMut<Board>,
    block: Block,
    all_blocks: &Vec<Block>,
    dir: Direction,
    next_blocks: &mut Vec<Block>,
    blocks_to_try_move: &mut Vec<Block>,
    blocks_that_must_move: &mut Vec<Block>,
) -> bool {
    for &position in block.positions.iter() {
        if dir != Direction::Up && dir != Direction:: Down {
            let mut next_position = position.position_above();
            while board.get_object_type(next_position) != GameObject::Empty {
                blocks_to_try_move.push(board.get_block(next_position));
                next_position = next_position.position_above();
            }
        }
        let can_current_block_move_somehow = is_moveable(board.get_object_type(position), false, dir)
            && board.get_floor_type(position.position_below()) == Floor::Ice;
        if !can_current_block_move_somehow {
            return false;
        }

        let mut next_block = block.clone();
        let mut next_position = position;
        let mut curr_position;
        while next_block == block {
            curr_position = next_position;
            next_position = board
                .get_next_position_for_move(curr_position, dir);
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
            blocks_to_try_move,
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

pub fn move_weak(
    board: &mut ResMut<Board>,
    block: Block,
    all_blocks: &Vec<Block>,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
) -> bool {
    let mut next_blocks = Vec::new();
    let mut blocks_that_must_move = Vec::new();
    let mut blocks_to_try_move = Vec::new();
    let can_block_move = can_block_move_weak(
        board,
        block.clone(),
        all_blocks,
        direction,
        &mut next_blocks,
        &mut blocks_to_try_move,
        &mut blocks_that_must_move,
    );
    let was_moved = can_block_move || !blocks_that_must_move.is_empty();
    perform_move(blocks_that_must_move, board, direction, writer, true);
    if can_block_move {
        perform_move(next_blocks, board, direction, writer, true);
    }
    for block in blocks_to_try_move.iter() {
        move_weak(board, block.clone(), all_blocks, direction, writer);
    }
    return was_moved;
}
