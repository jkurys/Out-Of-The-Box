use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::board::Board;
use crate::game::game_objects::{Position, Block};
use crate::state::MoveState;

use super::events::EnteredFloorEvent;
use super::resources::FireAnimation;
use super::utils::{can_block_move, move_strong};

//BUG: weird stuff happens when a block player eats a block box
//BUG: spitting disappears a turtle

pub fn handle_spit(
    mut board: ResMut<Board>,
    mut writer: EventWriter<EnteredFloorEvent>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut fire_animation: ResMut<FireAnimation>,
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
        spit_out(position, &mut writer, &mut board, &mut app_state, &mut fire_animation);
    }
}

pub fn spit_out(
    position: Position,
    writer: &mut EventWriter<EnteredFloorEvent>,
    board: &mut ResMut<Board>,
    app_state: &mut ResMut<NextState<MoveState>>,
    fire_animation: &mut ResMut<FireAnimation>,
) {
    let mut empty_vec = Vec::new();
    let mut empty_vec2 = Vec::new();
    let (obj, dir) = board.get_eat_data(position);
    let new_pos = position.next_position(dir);
    let can_push = can_block_move(board, board.get_block(new_pos), dir, &mut empty_vec, &mut empty_vec2)
        && can_block_move(board, board.get_block(position), dir, &mut empty_vec, &mut empty_vec2);
    // NOTE: here it should be the eaten block
    let can_push_backwards = can_block_move(board, Block { positions: HashSet::from([position]) }, dir.opposite(), &mut empty_vec, &mut empty_vec2);
    // NOTE: here it should be the player block
    let mut block = board.get_block(position);
    if can_push {
        board.remove_from_block(&mut block, position);
        let next_block = board.get_block(new_pos);
        move_strong(board, next_block, new_pos, dir, writer);
    }
    else if can_push_backwards {
        board.remove_from_block(&mut block, position);
        move_strong(board, board.get_block(position), position, dir.opposite(), writer);
    }
    if can_push {
        board.insert_object(new_pos, obj);
        writer.send(EnteredFloorEvent {
            floor: board.get_floor_type(new_pos),
            position: new_pos,
            object: obj,
            direction: dir,
        });
        if block.positions.len() != 0 {
            block.positions = block.positions
                .iter()
                .map(|p| p.next_position(dir))
                .collect();
            board.add_to_block(&mut block, new_pos);
        }
        board.remove_eat(position);
        app_state.set(MoveState::Animation);
        fire_animation.0 = true;
    }
    else if can_push_backwards {
        board.insert_object(position, obj);
        if block.positions.len() != 0 {
            board.add_to_block(&mut block, position);
        }
        board.remove_eat(position.next_position(dir.opposite()));
        app_state.set(MoveState::Animation);
        fire_animation.0 = true;
    }
}
