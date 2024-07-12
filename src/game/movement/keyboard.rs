use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::board::Board;
use crate::game::game_objects::{Block, Direction};
use crate::game::resources::PlayerDirection;
use crate::state::MoveState;

use super::resources::{MoveData, PushAttempt};

pub fn handle_keypress(
    board: Res<Board>,
    mut player_dir: ResMut<PlayerDirection>,
    move_data: ResMut<MoveData>,
    app_state: ResMut<NextState<MoveState>>,
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
) {
    let direction = if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        Direction::Up
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        Direction::Down
    } else if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        Direction::Right
    } else {
        return;
    };
    *player_dir = PlayerDirection(direction);
    move_player(board, move_data, app_state, direction);
}

pub fn move_player(
    board: Res<Board>,
    mut move_data: ResMut<MoveData>,
    mut app_state: ResMut<NextState<MoveState>>,
    direction: Direction,
) {
    let mut positions = board.get_player_positions();
    positions.sort_by(|&pos1, &pos2| match direction {
        Direction::Down => pos1.y.cmp(&pos2.y),
        Direction::Left => pos1.x.cmp(&pos2.x),
        Direction::Right => pos2.x.cmp(&pos1.x),
        Direction::Up => pos2.y.cmp(&pos1.y),
    });
    for position in positions {
        move_data.push_atempts.push(PushAttempt {
            block: Block {
                positions: HashSet::from([position]),
            },
            direction,
            is_weak: false,
            insert_after: None,
        });
    }
    app_state.set(MoveState::Calculating);
}
