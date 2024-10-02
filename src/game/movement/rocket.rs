use bevy::prelude::*;
use crate::{board::Board, game::game_objects::{Position, GameObject}, state::MoveState};

use super::{events::{EnteredFloorEvent, TryMoveEvent}, resources::{DisplayButton, FireAnimation}, spit::spit_out_far};

pub fn execute_rocket(
    board: &mut ResMut<Board>,
    position: Position,
    writer: &mut EventWriter<TryMoveEvent>,
    writer2: &mut EventWriter<EnteredFloorEvent>,
    app_state: &mut ResMut<NextState<MoveState>>,
    display_button: &mut ResMut<DisplayButton>,
    fire_animation: &mut ResMut<FireAnimation>,
) {
    let player = board.get_object_type(position);
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
        spit_out_far(position, writer, writer2, board, app_state, fire_animation, display_button);
        
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
