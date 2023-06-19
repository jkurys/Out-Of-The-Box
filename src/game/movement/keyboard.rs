use bevy::prelude::*;

use crate::board::Board;
use crate::game::game_objects::Direction;
use crate::state::MoveState;

use super::events::TryMoveEvent;

pub fn handle_keypress(
    keyboard_input: ResMut<Input<KeyCode>>,
    board: Res<Board>,
    mut writer: EventWriter<TryMoveEvent>,
    mut app_state: ResMut<NextState<MoveState>>,
) {
    let direction = if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        Direction::Up
    } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        Direction::Down
    } else if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        Direction::Right
    } else {
        return;
    };
    let mut positions = board.get_player_positions();
    positions.sort_by(|&pos1, &pos2| match direction {
        Direction::Down => pos1.y.cmp(&pos2.y),
        Direction::Left => pos1.x.cmp(&pos2.x),
        Direction::Right => pos2.x.cmp(&pos1.x),
        Direction::Up => pos2.y.cmp(&pos1.y),
    });
    for position in positions {
        writer.send(TryMoveEvent {
            position,
            direction,
            is_weak: false,
            insert_after: None,
        });
    }
    app_state.set(MoveState::Calculating);
}
