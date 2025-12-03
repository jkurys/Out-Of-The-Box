use bevy::prelude::*;

use crate::board::GameData;
use crate::game::game_objects::{Block, Position, SmallSet};
use crate::state::MoveState;

use super::events::EnteredFloorEvent;
use super::resources::FireAnimation;
use super::strong::get_affected_blocks;
use super::utils::perform_move;

pub fn handle_spit(
    mut game_data: ResMut<GameData>,
    mut writer: EventWriter<EnteredFloorEvent>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut fire_animation: ResMut<FireAnimation>,
) {
    let board = &mut game_data.board;
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
        spit_out(
            position,
            &mut writer,
            &mut game_data,
            &mut app_state,
            &mut fire_animation,
        );
    }
}

fn spit_forwards(
    position: Position,
    game_data: &mut ResMut<GameData>,
    writer: &mut EventWriter<EnteredFloorEvent>,
    app_state: &mut ResMut<NextState<MoveState>>,
    fire_animation: &mut ResMut<FireAnimation>,
    blocks_to_move: &mut Vec<Block>,
) {
    let (obj, floor_opt, dir) = game_data.board.get_eat_data(position);
    let new_pos = position.next_position(dir);
    let old_obj = game_data.board.get_object_type(position);
    {
        // here we introduce a scope to avoid borrow issues
        let GameData {
            board,
            entity_storage,
        } = &mut **game_data;
        board.delete_object(position, entity_storage);
        board.insert_object(position, obj);
        if let Some(floor) = floor_opt {
            board.insert_floor(new_pos, floor);
        }
        let block = board.get_block(position);
        blocks_to_move.push(block.clone());
    }
    perform_move(blocks_to_move.to_vec(), game_data, dir, writer, false);
    let GameData {
        board,
        entity_storage: _,
    } = &mut **game_data;
    board.insert_object(position, old_obj);
    // this ensures that the animation is played
    writer.write(EnteredFloorEvent {
        floor: board.get_floor_type(new_pos),
        position: new_pos,
        object: obj,
        direction: dir,
    });
    board.remove_eat(position);
    app_state.set(MoveState::Animation);
    fire_animation.0 = true;
}

fn spit_backwards(
    position: Position,
    game_data: &mut ResMut<GameData>,
    writer: &mut EventWriter<EnteredFloorEvent>,
    app_state: &mut ResMut<NextState<MoveState>>,
    fire_animation: &mut ResMut<FireAnimation>,
    blocks_to_move: &mut Vec<Block>,
) {
    let (obj, floor_opt, dir) = game_data.board.get_eat_data(position);
    perform_move(
        blocks_to_move.to_vec(),
        game_data,
        dir.opposite(),
        writer,
        false,
    );
    let GameData {
        board,
        entity_storage,
    } = &mut **game_data;
    let old_obj = board.get_object_type(position);
    board.delete_object(position, entity_storage);
    board.insert_object(position, obj);
    if let Some(floor) = floor_opt {
        board.insert_floor(position, floor);
    }
    let next_pos = position.next_position(dir.opposite());
    board.insert_object(next_pos, old_obj);
    writer.write(EnteredFloorEvent {
        floor: board.get_floor_type(next_pos),
        position: next_pos,
        object: old_obj,
        direction: dir.opposite(),
    });
    board.remove_eat(next_pos);
    app_state.set(MoveState::Animation);
    fire_animation.0 = true;
}

pub fn spit_out(
    position: Position,
    writer: &mut EventWriter<EnteredFloorEvent>,
    game_data: &mut ResMut<GameData>,
    app_state: &mut ResMut<NextState<MoveState>>,
    fire_animation: &mut ResMut<FireAnimation>,
) {
    let board = &mut game_data.board;
    let (_, _, dir) = board.get_eat_data(position);
    let new_pos = position.next_position(dir);
    let mut blocks_to_move = Vec::new();
    let mut blocks_to_move_backwards = Vec::new();
    let block = board.get_block(position);
    let new_pos_block = board.get_block(new_pos);
    let can_push = get_affected_blocks(
        game_data,
        new_pos_block,
        dir,
        &mut blocks_to_move,
        &mut Vec::new(),
        &mut Vec::new(),
    ) && get_affected_blocks(
        game_data,
        block,
        dir,
        &mut blocks_to_move,
        &mut Vec::new(),
        &mut Vec::new(),
    );
    let can_push_backwards = get_affected_blocks(
        game_data,
        Block {
            positions: SmallSet::from([position]),
        },
        dir.opposite(),
        &mut blocks_to_move_backwards,
        &mut Vec::new(),
        &mut Vec::new(),
    );
    if can_push {
        spit_forwards(
            position,
            game_data,
            writer,
            app_state,
            fire_animation,
            &mut blocks_to_move,
        );
    } else if can_push_backwards {
        spit_backwards(
            position,
            game_data,
            writer,
            app_state,
            fire_animation,
            &mut blocks_to_move_backwards,
        );
    }
}
