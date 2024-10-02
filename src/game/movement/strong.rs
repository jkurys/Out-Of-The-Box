use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::Board,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
};

use super::{events::EnteredFloorEvent, resources::DisplayButton, utils::{perform_move, is_moveable}, eat::perform_eat, powerup::eat_powerup};

pub fn can_block_move(
    board: &ResMut<Board>,
    block: Block,
    dir: Direction,
    next_blocks: &mut Vec<Block>,
    blocks_to_try_move: &mut Vec<Block>,
    visited_blocks: &mut Vec<Block>,
) -> bool {
    let is_first = visited_blocks.is_empty();
    visited_blocks.push(block.clone());
    for &position in block.positions.iter() {
        if dir != Direction::Up && dir != Direction:: Down {
            let mut next_position = position.position_above();
            while board.get_object_type(next_position) != GameObject::Empty
                && is_moveable(board.get_object_type(next_position), false, dir) {
                blocks_to_try_move.push(board.get_block(next_position));
                next_position = next_position.position_above();
            }
        }
        if board.get_object_type(position) == GameObject::Empty {
            continue;
        }
        if !is_moveable(board.get_object_type(position), is_first, dir) {
            // eat if not only player
            return false;
        }
        let next_position = board.get_next_position_for_move(position, dir);
        let next_block = board.get_block(next_position);
        if board.is_block_empty(&next_block) 
            || board.get_object_type(next_position) == GameObject::Empty {
            continue;
        }
        let can_move = visited_blocks.contains(&next_block)
        || can_block_move(
            board,
            next_block.clone(),
            dir,
            next_blocks,
            blocks_to_try_move,
            visited_blocks
        );
        if next_block != block && !can_move {
            return false;
        }
        if next_block != block {
            next_blocks.push(next_block)
        }
    }
    return true;
}

pub fn move_strong(
    board: &mut ResMut<Board>,
    block: Block,
    position: Position,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
    was_moved_already: bool,
    display_button: &mut ResMut<DisplayButton>,
) -> bool {

    let mut next_blocks = vec![block.clone()];
    let mut visited_blocks = Vec::new();
    let mut blocks_to_try_move = Vec::new();
    let next_pos = position.next_position(direction);
    if GameObject::Empty == board.get_object_type(position) {
        return false;
    }
    if let GameObject::PowerUp { powerup_type } = board.get_object_type(next_pos) {
        if matches!(board.get_object_type(position), GameObject::Player { powerup: _, direction: _ }) {
            eat_powerup(
                powerup_type,
                position,
                block,
                board,
                direction,
                writer,
                false,
                display_button,
            );
            return true;
        }
    }
    display_button.0 = false;
    if !can_block_move(
        board,
        block.clone(),
        direction,
        &mut next_blocks,
        &mut blocks_to_try_move,
        &mut visited_blocks,
    ) {
        if board.get_object_type(next_pos) == GameObject::Box
            && matches!(board.get_object_type(position), GameObject::Player { powerup: _, direction: _ })
            && board.get_eat_counter(position).is_none()
            && board.get_floor_type(next_pos.position_below()) != Floor::Dirt {
            // NOTE: otherwise turtles could eat objects
            // maybe they could in the future?
            perform_eat(board, block, next_pos, direction, writer, display_button);
            return true;
        }
        return false;
    }
    if !was_moved_already {
        let mut moved_positions = HashSet::new();
        blocks_to_try_move.sort_by(|block1, block2| block1.get_last_pos().z.cmp(&block2.get_last_pos().z));
        let mut can_move = true;
        for block in blocks_to_try_move.iter() {
            if !can_move {
                break;
            }
            let moved_clone = moved_positions.clone();
            let both: HashSet<&Position> = moved_clone.intersection(&block.positions).collect();
            if both.is_empty() {
                moved_positions = moved_positions.union(&block.positions).map(|&p| p).collect();
                can_move = move_strong(board, block.clone(), block.get_last_pos(), direction, writer, false, display_button);
            }
        }
    }
    perform_move(next_blocks, board, direction, writer, false);
    return true;
}
