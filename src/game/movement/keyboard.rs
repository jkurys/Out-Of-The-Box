use bevy::prelude::*;
use itertools::Itertools;

use crate::board::GameData;
use crate::game::game_objects::{Block, Direction, GameObject, Position};
use crate::state::MoveState;

use super::events::{TeleportEvent, TryMoveEvent};
use super::BoardPreMove;

fn handle_action_press(
    game_data: Res<GameData>,
    mut app_state: ResMut<NextState<MoveState>>,
    teleport_writer: &mut EventWriter<TeleportEvent>,
    mut board_pre_move: ResMut<BoardPreMove>,
) {
    let board = &game_data.board;
    let objects = board.get_objects();
    for (&position, &obj) in objects.iter() {
        if obj == GameObject::TeleBox {
            //in the case of multiple players, it could be cyclic;
            //what in case of a large player block?
            let player_pos = board.get_player_positions()[0];
            app_state.set(MoveState::TeleportAnimation);
            teleport_writer.send(TeleportEvent {
                position1: position,
                position2: player_pos,
            });
            return;
        }
    }
    board_pre_move.0 = board.clone();
    app_state.set(MoveState::Calculating);
}

pub fn handle_keypress(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    game_data: Res<GameData>,
    mut writer: EventWriter<TryMoveEvent>,
    mut teleport_writer: EventWriter<TeleportEvent>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut board_pre_move: ResMut<BoardPreMove>,
) {
    let board = &game_data.board;
    let direction = if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        Direction::North
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        Direction::South
    } else if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        Direction::Right
    } else {
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            handle_action_press(game_data, app_state, &mut teleport_writer, board_pre_move);
        }
        return;
    };
    let mut positions = board.get_player_positions();

    positions.sort_by(|&pos1, &pos2| match direction {
        Direction::South => pos1.y.cmp(&pos2.y),
        Direction::Left => pos1.x.cmp(&pos2.x),
        Direction::Right => pos2.x.cmp(&pos1.x),
        Direction::North => pos2.y.cmp(&pos1.y),
        Direction::Up => pos2.z.cmp(&pos1.z),
        Direction::Down => pos1.z.cmp(&pos2.z),
    });
    let blocks: Vec<(Block, Position)> = positions
        .into_iter()
        .map(|p| (board.get_block(p), p))
        .unique()
        .collect();

    for (block, position) in blocks {
        writer.send(TryMoveEvent {
            position,
            block,
            direction,
            is_weak: false,
            is_long: false,
        });
    }

    board_pre_move.0 = board.clone();
    app_state.set(MoveState::Calculating);
}
