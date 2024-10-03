use bevy::prelude::*;
use itertools::Itertools;

use crate::board::Board;
use crate::game::game_objects::{Block, Direction, Position, GameObject, PowerUpType};
use crate::state::MoveState;

use super::BoardPreMove;
use super::events::{TryMoveEvent, EnteredFloorEvent, TeleportEvent};
use super::resources::{FireAnimation, DisplayButton};
use super::rocket::execute_rocket;
use super::spit::spit_out;

pub fn handle_keypress(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    mut writer: EventWriter<TryMoveEvent>,
    mut writer2: EventWriter<EnteredFloorEvent>,
    mut writer3: EventWriter<TeleportEvent>,
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
        let objects = board.get_objects();
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            for (&position, &obj) in objects.iter() {
                if obj == GameObject::TeleBox{
                    //in the case of multiple players, it could be cyclic;
                    //what in case of a large player block?
                    let player_pos = board.get_player_positions()[0];
                    app_state.set(MoveState::TeleportAnimation);
                    writer3.send(TeleportEvent {
                        position1: position,
                        position2: player_pos,
                    });
                    return;
                }
            }
            for &position in positions.iter() {
                let player = board.get_object_type(position);
                if matches!(player, GameObject::Player { powerup: Some(PowerUpType::Rocket), direction: _ }) {
                    execute_rocket(&mut board, position, &mut writer, &mut writer2, &mut app_state, &mut display_button, &mut fire_animation);
                }
                if matches!(player, GameObject::Player { powerup: Some(PowerUpType::Teleport), direction: _ }) {
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
                    if has_box {
                        let (_obj, floor_opt, dir) = board.get_eat_data(position);
                        board.remove_eat(position);
                        board.insert_eat(position, dir, GameObject::TeleBox, floor_opt);
                        spit_out(position, &mut writer2, &mut board, &mut app_state, &mut fire_animation, &mut display_button);
                    } else {
                        let position_to_land = position.next_position(direction).next_position(direction);
                        if board.get_object_type(position_to_land) == GameObject::Empty {
                            board.delete_object(position);
                            board.insert_object(position_to_land, GameObject::Player { powerup: None, direction });
                        } 
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
