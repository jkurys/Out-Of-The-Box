use bevy::prelude::*;

use super::{events::EnteredFloorEvent, resources::AnimationTimer};
use crate::{
    game::game_objects::{Floor, GameObject},
    state::CurrentMap, resources::Board,
};


pub fn handle_warp(
    mut current_map: ResMut<State<CurrentMap>>,
    mut moved: EventReader<EnteredFloorEvent>,
    mut board: ResMut<Board>,
    timer: Res<AnimationTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    for event in moved.iter() {
        let position = event.position;
        if let Floor::Warp(map) = event.floor {
            if event.object == GameObject::Player || event.object == GameObject::Box {
                let curr_map = board.get_current_map();
                board.delete_object(position, curr_map);
                let warp_position = board.get_warp_position(map, board.get_current_map());
                board.insert_object_to_map(warp_position, event.object, map);
            }
            if event.object == GameObject::Player {
                current_map
                    .push(CurrentMap(Some(map)))
                    .expect("Could not switch maps state");
                board.set_current_map(map);
            }
        }
    }
}
