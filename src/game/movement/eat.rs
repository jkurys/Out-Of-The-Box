use bevy::prelude::*;

use crate::{
    board::GameData,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
};

use super::{events::EnteredFloorEvent, strong::move_strong, utils::perform_move};

pub fn perform_eat(
    game_data: &mut ResMut<GameData>,
    block: Block,
    next_pos: Position,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
) {
    let GameData {
        board,
        entity_storage,
    } = &mut **game_data;
    board.delete_object(next_pos, entity_storage);
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
    for &block in blocks_to_try_move.iter() {
        move_strong(game_data, block, direction, writer);
    }
    perform_move([block].to_vec(), game_data, direction, writer, false);
}
