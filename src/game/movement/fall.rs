use bevy::prelude::*;

use crate::board::Board;
use crate::game::game_objects::{Block, Floor, GameObject, Position};

pub fn handle_fall(
    mut board: ResMut<Board>,

) {
    let mut void_positions = board.get_all_positions(Floor::Void);
    let mut empty_positions = board.get_empty_below();
    void_positions.append(&mut empty_positions);
    void_positions = void_positions.iter()
        .map(|&p| p.position_above())
        .collect();
    for pos in void_positions.clone().iter() {
        let mut pos_above = pos.position_above();
        while board.get_object_type(pos_above) != GameObject::Empty {
            void_positions.push(pos_above);
            pos_above = pos_above.position_above();
        }
    }
    let mut blocks: Vec<Block> = void_positions
        .iter()
        .filter(|&&p| board.get_object_type(p) != GameObject::Empty
            && !matches!(board.get_object_type(p), GameObject::HidingWall { color: _, hidden_toggle: _, hidden_by_def: _ } ))
        .map(|&p| board.get_block(p))
        .collect();
    blocks.sort_by(|block1, block2| {
        let mut lowest_z1 = 1000;
        let mut lowest_z2 = 1000;
        for position in block1.positions.iter() {
            if position.z < lowest_z1 {
                lowest_z1 = position.z;
            }
        }
        for position in block2.positions.iter() {
            if position.z < lowest_z2 {
                lowest_z2 = position.z;
            }
        }
        return lowest_z1.cmp(&lowest_z2);
    });
    for block in blocks {
        fall_block(
            &mut board,
            block
        );
    }
}

fn fall_block(
    board: &mut ResMut<Board>,
    mut block: Block,
) {
    let mut can_fall = true;
    for position in block.positions.iter() {
        if board.get_floor_type(position.position_below()) != Floor::Void
            && board.get_object_type(position.position_below()) != GameObject::Empty {
            can_fall = false;
        }
    }
    let mut i = 0;
    while can_fall && i < 10 {
        block = board.fall_block(block);
        for position in block.positions.iter() {
            if board.get_floor_type(position.position_below()) != Floor::Void
                && board.get_object_type(position.position_below()) != GameObject::Empty {
                can_fall = false;
            }
        }
        i += 1;
    }
}
