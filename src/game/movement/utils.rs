use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    board::Board,
    game::game_objects::{Block, Direction, GameObject, Position},
};

use super::{events::EnteredFloorEvent, sort_positions::sort_positions};

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
            | GameObject::Player {
                powerup: _,
                direction: _,
            }
            | GameObject::Turtle {
                direction: _,
                color: _,
            }
            | GameObject::TurtleHead {
                direction: _,
                color: _,
            }
            | GameObject::Empty
            | GameObject::PowerUp { powerup_type: _ }
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
    board: &mut ResMut<Board>,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
    is_weak: bool,
) {
    let mut positions_vec: Vec<Position> = blocks
        .iter()
        .map(|block| block.positions.clone())
        .flatten()
        .unique()
        .collect();
    positions_vec.sort_by(sort_positions(direction));
    for position in positions_vec {
        if is_weak {
            board.move_object_no_countdown(position, direction);
        }
        else {
            board.move_object(position, direction);
        }
        let next_position = board.get_next_position_for_move(position, direction);
        writer.send(EnteredFloorEvent {
            floor: board.get_floor_type(next_position),
            position: next_position,
            object: board.get_object_type(next_position),
            direction,
        });
    }
}

