use bevy::prelude::*;

use crate::board::Board;
use crate::game::game_objects::{Block, Floor, GameObject};

pub fn handle_fall(
    mut board: ResMut<Board>,

) {
    let mut void_positions = board.get_all_positions(Floor::Void);
    let mut empty_positions = board.get_empty_below();
    void_positions.append(&mut empty_positions);
    let blocks: Vec<Block> = void_positions
        .iter()
        .filter(|&p| board.get_object_type(p.position_above()) != GameObject::Empty)
        .map(|&p| board.get_block(p.position_above()))
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
        if board.get_floor_type(position.position_below()) != Floor::Void
            && board.get_object_type(position.position_below()) != GameObject::Empty {
            can_fall = false;
            println!("XDD");
        }
    }
    if can_fall {
            println!("XDDDD");
        board.fall_block(block);
    }
}
