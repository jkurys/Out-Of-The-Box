use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::Board,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
};

use super::{events::EnteredFloorEvent, resources::DisplayButton, strong::move_strong, utils::perform_move};

pub fn perform_eat(
    board: &mut ResMut<Board>,
    block: Block,
    next_pos: Position,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
    display_button: &mut ResMut<DisplayButton>,
) {
    board.delete_object(next_pos);
    let pos = next_pos.next_position(direction.opposite());
    let floor = board.get_floor_type(next_pos);
    board.delete_floor(next_pos);
    let floor_opt;
    if floor == Floor::Tile {
        floor_opt = None;
    } else {
        floor_opt = Some(floor);
    }
    board.insert_eat(pos, direction, GameObject::Box, floor_opt);
    let position = next_pos.prev_position(direction);
    let mut blocks_to_try_move = Vec::new();
    if direction != Direction::Up {
        let mut next_position = position.position_above();
        while board.get_object_type(next_position) != GameObject::Empty {
            blocks_to_try_move.push(board.get_block(next_position));
            next_position = next_position.position_above();
        }
    }
    let moved_positions = HashSet::new();
    for block in blocks_to_try_move.iter() {
        let moved_copy = moved_positions.clone();
        moved_copy.intersection(&block.positions);
        if moved_copy.is_empty() {
            moved_positions.union(&block.positions);
            move_strong(board, block.clone(), block.get_last_pos(), direction, writer, false, display_button);
        }
    }
    perform_move([block].to_vec(), board, direction, writer, false);
}
