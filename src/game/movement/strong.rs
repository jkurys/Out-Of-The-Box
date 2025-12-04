use bevy::prelude::*;

use crate::{
    board::GameData,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
};

use super::{
    eat::perform_eat,
    events::EnteredFloorMessage,
    utils::{is_moveable, perform_move},
};

pub fn can_block_move(game_data: &mut ResMut<GameData>, block: Block, dir: Direction) -> bool {
    let mut next_blocks = vec![];
    let mut visited_blocks = Vec::new();
    let mut blocks_to_try_move = Vec::new();
    get_affected_blocks(
        game_data,
        block,
        dir,
        &mut next_blocks,
        &mut blocks_to_try_move,
        &mut visited_blocks,
    )
}

/*
 * This ensures that blocks that are on top of the moving block are moved as well
*/
pub fn add_blocks_above_to_move(
    game_data: &ResMut<GameData>,
    position: Position,
    dir: Direction,
    blocks_to_try_move: &mut Vec<Block>,
) {
    let board = &game_data.board;
    if dir != Direction::Up && dir != Direction::Down {
        let mut next_position = position.position_above();
        let mut next_object = board.get_object_type(next_position);
        while next_object != GameObject::Empty && is_moveable(next_object, false, dir) {
            blocks_to_try_move.push(board.get_block(next_position));
            next_position = next_position.position_above();
            next_object = board.get_object_type(next_position);
        }
    }
}

/*
 * Returns whether the starting block can move in the given direction
 * along with 3 vectors:
 * - blocks_that_have_to_move: all blocks that have to move if the starting block moves
 * - blocks_to_try_move: all blocks that should be tried to move
 *      as a consequence of moving the starting block
 * - visited_blocks: all blocks that have been visited during the recursion
 *      to avoid infinite recursion
*/

pub fn get_affected_blocks(
    game_data: &mut ResMut<GameData>,
    starting_block: Block,
    dir: Direction,
    blocks_that_have_to_move: &mut Vec<Block>,
    blocks_to_try_move: &mut Vec<Block>,
    visited_blocks: &mut Vec<Block>,
) -> bool {
    let is_first = visited_blocks.is_empty();
    visited_blocks.push(starting_block.clone());
    for position in starting_block.positions.iter() {
        let moving_object_type = game_data.board.get_object_type(position);
        add_blocks_above_to_move(game_data, position, dir, blocks_to_try_move);
        if !is_moveable(moving_object_type, is_first, dir) {
            return false;
        }
        let next_position = game_data.board.get_next_position_for_move(position, dir);
        let next_block = game_data.board.get_block(next_position);
        if moving_object_type == GameObject::Empty
            || game_data.board.get_object_type(next_position) == GameObject::Empty
            || next_block == starting_block
        {
            continue;
        }
        // if visited_blocks contains next_block,
        // we have already checked it and returned true
        let can_move = visited_blocks.contains(&next_block)
            || get_affected_blocks(
                game_data,
                next_block.clone(),
                dir,
                blocks_that_have_to_move,
                blocks_to_try_move,
                visited_blocks,
            );
        if !can_move {
            return false;
        }
        blocks_that_have_to_move.push(next_block)
    }
    return true;
}

fn try_eat(
    game_data: &mut ResMut<GameData>,
    block: Block,
    direction: Direction,
    writer: &mut MessageWriter<EnteredFloorMessage>,
) -> bool {
    let board = &mut game_data.board;
    for position in block.positions.iter() {
        let next_pos = position.next_position(direction);
        let current_object_type = board.get_object_type(position);
        let next_object_type = board.get_object_type(next_pos);
        if next_object_type == GameObject::Box
            && matches!(current_object_type, GameObject::Player { direction: _ })
            && board.get_eat_counter(position).is_none()
            && board.get_floor_type(next_pos.position_below()) != Floor::Dirt
        {
            // NOTE: otherwise turtles could eat objects
            // maybe they could in the future?
            perform_eat(game_data, block, next_pos, direction, writer);
            return true;
        }
    }
    return false;
}

pub fn move_strong(
    game_data: &mut ResMut<GameData>,
    block: Block,
    direction: Direction,
    writer: &mut MessageWriter<EnteredFloorMessage>,
) -> bool {
    let mut blocks_that_move = vec![block.clone()];
    let mut visited_blocks = Vec::new();
    let mut blocks_to_try_move = Vec::new();
    let can_move = get_affected_blocks(
        game_data,
        block.clone(),
        direction,
        &mut blocks_that_move,
        &mut blocks_to_try_move,
        &mut visited_blocks,
    );
    if !can_move {
        return try_eat(game_data, block, direction, writer);
    }
    // maybe was_moved_already could be removed if we remove this possibility somehow

    for block in blocks_to_try_move.iter() {
        move_strong(game_data, block.clone(), direction, writer);
    }
    perform_move(blocks_that_move, game_data, direction, writer, false);
    return true;
}
