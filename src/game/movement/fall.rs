use bevy::prelude::*;

use crate::board::Board;
use crate::game::game_objects::{Block, Floor, GameObject};

pub fn handle_fall(
    mut board: ResMut<Board>,

) {
    let void_positions = board.get_all_positions(Floor::Void);
    let blocks: Vec<Block> = void_positions
        .iter()
        .filter(|&p| board.get_object_type(*p) != GameObject::Empty)
        .map(|&p| board.get_block(p))
        .collect();
    for block in blocks {
        fall_block(
            &mut board,
            block
        );
    }
}

fn fall_block(
    board: &mut ResMut<Board>,
    block: Block,
) {
    let mut can_fall = true;
    for position in block.positions.iter() {
        if board.get_floor_type(*position) != Floor::Void {
            can_fall = false;
        }
    }
    if can_fall {
        board.fall_block(block);
    }
}
