use bevy::prelude::*;

use super::resources::{AnimationTimer, MoveData};
use crate::{
    board::Board,
    game::game_objects::{Floor, GameObject},
};

pub fn handle_warp(
    // mut moved: EventReader<EnteredFloorEvent>,
    move_data: Res<MoveData>,
    mut board: ResMut<Board>,
    timer: Res<AnimationTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    for event in move_data.moves.iter() {
        let position = event.position;
        if let Floor::Warp(map) = event.floor {
            if event.object == GameObject::Player || event.object == GameObject::Box {
                board.delete_object(position);
                let warp_position = board.get_warp_position(map, board.get_current_map());
                board.insert_object_to_map(warp_position, event.object, map);
            }
            if event.object == GameObject::Player {
                board.set_current_map(map);
            }
        }
    }
}
