use bevy::prelude::*;
use itertools::Itertools;

use crate::board::Board;
use crate::game::game_objects::{Block, Direction, Position, GameObject, PowerUpType};
use crate::state::MoveState;

use super::BoardPreMove;
use super::events::{TryMoveEvent, EnteredFloorEvent};
use super::resources::{FireAnimation, DisplayButton};
use super::spit::spit_out_far;

pub fn handle_keypress(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    mut writer: EventWriter<TryMoveEvent>,
    mut writer2: EventWriter<EnteredFloorEvent>,
    mut app_state: ResMut<NextState<MoveState>>,
    mut board_pre_move: ResMut<BoardPreMove>,
    mut fire_animation: ResMut<FireAnimation>,
    mut display_button: ResMut<DisplayButton>,
) {
    let direction = if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        Direction::North
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        Direction::South
    } else if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        Direction::Right
    } else {
        let positions = board.get_player_positions();
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            for &position in positions.iter() {
                let player = board.get_object_type(position);
                if matches!(player, GameObject::Player { powerup: Some(PowerUpType::Rocket), direction: _ }) {
                    let GameObject::Player { powerup: _, direction } = player else {
                        panic!("Player was not player!");
                    };
                    let eaten = board.get_all_eat();
                    let mut has_box = false;
                    for (&eat_pos, _) in eaten.iter() {
                        if eat_pos == position {
                            has_box = true;
                        }
                    }
                    board.delete_object(position);
                    board.insert_object(position, GameObject::Player { powerup: None, direction });
                    if has_box {
                        spit_out_far(position, &mut writer, &mut writer2, &mut board, &mut app_state, &mut fire_animation, &mut display_button);
                        
                    } else {
                        writer.send(TryMoveEvent {
                            position,
                            block: board.get_block(position),
                            direction,
                            is_weak: false,
                            is_long: true,
                        });
                    }
                }
            }
            board_pre_move.0 = board.clone();
            app_state.set(MoveState::Calculating);
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
