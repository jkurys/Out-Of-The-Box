use bevy::prelude::*;

use crate::board::GameData;
use crate::game::game_objects::{Block, Direction, Floor, GameObject, Position};

use super::events::TryMoveMessage;
use super::utils::is_moveable;

pub fn handle_fall(mut game_data: ResMut<GameData>, mut writer: MessageWriter<TryMoveMessage>) {
    let board = &game_data.board;
    let mut void_positions = board.get_positions_to_fall();
    void_positions = void_positions.iter().map(|&p| p.position_above()).collect();
    for pos in void_positions.clone().iter() {
        let mut pos_above = pos.position_above();
        while board.get_object_type(pos_above) != GameObject::Empty {
            void_positions.push(pos_above);
            pos_above = pos_above.position_above();
        }
    }
    let mut blocks: Vec<Block> = void_positions
        .iter()
        .filter(|&&p| {
            board.get_object_type(p) != GameObject::Empty
                && !matches!(
                    board.get_object_type(p),
                    GameObject::HidingWall {
                        color: _,
                        hidden_toggle: _,
                        hidden_by_def: _
                    }
                )
        })
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
    for block in blocks.iter() {
        fall_block(&mut game_data, block.clone(), &mut writer);
    }
}

fn fall_block(
    game_data: &mut ResMut<GameData>,
    block: Block,
    writer: &mut MessageWriter<TryMoveMessage>,
) {
    let board = &mut game_data.board;
    let mut can_fall = true;
    let mut last_position = Position { x: 0, y: 0, z: 0 };
    for position in block.positions.iter() {
        last_position = position;
        if board.get_floor_type(position.position_below()) != Floor::Void
            && board.get_object_type(position.position_below()) != GameObject::Empty
        {
            can_fall = false;
        }
    }
    let mut next_position = last_position.next_position(Direction::Down);
    while is_moveable(board.get_object_type(next_position), false, Direction::Down)
        && board.get_object_type(next_position) != GameObject::Empty
    {
        last_position = last_position.next_position(Direction::Up);
        next_position = next_position.next_position(Direction::Up);
    }
    if can_fall {
        writer.write(TryMoveMessage {
            block,
            direction: Direction::Down,
            is_weak: false,
            is_long: true,
            position: last_position,
        });
    }
}
