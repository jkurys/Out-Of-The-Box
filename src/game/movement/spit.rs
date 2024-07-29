use bevy::prelude::*;

use crate::board::Board;
use crate::game::game_objects::{Position, GameObject};

use super::events::TryMoveEvent;
use super::utils::can_block_move;

pub fn handle_spit(
    mut board: ResMut<Board>,
    mut writer: EventWriter<TryMoveEvent>,
) {
    let is_zeroed = |p| {
        let counter_opt = board.get_eat_counter(p);
        if counter_opt.is_none() {
            return false;
        }
        if counter_opt.unwrap() == 0 {
            return true;
        }
        return false;
    };
    let positions = board.get_player_positions();
    let counter_zeroed: Vec<Position> = positions
        .clone()
        .into_iter()
        .filter(|p| is_zeroed(*p))
        .collect();
    for position in counter_zeroed {
        spit_out(position, &mut writer, &mut board);
    }
}

pub fn spit_out(
    position: Position,
    writer: &mut EventWriter<TryMoveEvent>,
    board: &mut ResMut<Board>,
) {
    let mut empty_vec = Vec::new();
    let mut empty_vec2 = Vec::new();
    let (obj, dir) = board.get_eat_data(position);
    let new_pos = position.next_position(dir);
    let next_obj = board.get_object_type(new_pos);
    match next_obj {
        GameObject::Empty => {
            board.insert_object(new_pos, obj);
            board.remove_eat(position);
        },
        _ => {
            if can_block_move(board, board.get_block(new_pos), dir, &mut empty_vec, &mut empty_vec2) {
                writer.send(TryMoveEvent {
                    block: board.get_block(new_pos),
                    position: new_pos,
                    direction: dir,
                    is_weak: false,
                });
            }
            else {
                writer.send(TryMoveEvent {
                    block: board.get_block(position),
                    position,
                    direction: dir.opposite(),
                    is_weak: false,
                });
            }
        }
    };
    
}
