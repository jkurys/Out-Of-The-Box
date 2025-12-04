use bevy::prelude::*;

use crate::board::GameData;
use crate::game::game_objects::{Block, Direction, GameObject, Position};

use super::{events::EnteredFloorMessage, sort_positions::sort_positions};

/*
 * Returns whether the given object is moveable in the given direction.
 * is_first is needed, since HidingWalls can only be moved
 * by pressing a button, which makes them the object initiating the move.
 */
pub fn is_moveable(obj: GameObject, is_first: bool, direction: Direction) -> bool {
    if is_first {
        if let GameObject::HidingWall { .. } = obj {
            if direction == Direction::Up || direction == Direction::Down {
                return true;
            }
        }
    }
    matches!(
        obj,
        GameObject::Box
            | GameObject::Player { direction: _ }
            | GameObject::Turtle {
                direction: _,
                color: _,
            }
            | GameObject::TurtleHead {
                direction: _,
                color: _,
            }
            | GameObject::Empty
            | GameObject::TeleBox,
    )
}

pub fn is_position_in_blocks(blocks: &Vec<Block>, position: Position) -> bool {
    for block in blocks {
        if block.contains_position(position) {
            return true;
        }
    }
    false
}

pub fn perform_move(
    blocks: Vec<Block>,
    game_data: &mut ResMut<GameData>,
    direction: Direction,
    writer: &mut MessageWriter<EnteredFloorMessage>,
    is_weak: bool,
) {
    let board = &mut game_data.board;
    let mut positions_vec: Vec<Position> = blocks
        .iter()
        .flat_map(|block| block.positions.iter())
        .collect();
    positions_vec.sort_by(sort_positions(direction));
    for position in positions_vec {
        if is_weak {
            board.move_object_no_countdown(position, direction);
        } else {
            board.move_object(position, direction);
        }
        let next_position = board.get_next_position_for_move(position, direction);
        writer.write(EnteredFloorMessage {
            floor: board.get_floor_type(next_position),
            position: next_position,
            object: board.get_object_type(next_position),
            direction,
        });
    }
    for block in &blocks {
        board.move_block(block, direction);
    }
}
