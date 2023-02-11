use bevy::prelude::*;

use super::events::{EnteredFloorEvent, ExitedFloorEvent};
use crate::game::game_objects::{Floor, GameObject};
use crate::game::resources::Board;

use super::resources::AnimationTimer;
// checks which entities should move if they are on ice
pub fn handle_ice(
    mut moved_writer: EventWriter<ExitedFloorEvent>,
    mut position_reader: EventReader<EnteredFloorEvent>,
    timer: ResMut<AnimationTimer>,
    board: Res<Board>,
) {
    if !timer.0.finished() {
        return;
    }
    let mut events = Vec::new();
    let mut positions = Vec::new();
    for event in position_reader.iter() {
        positions.push(event.position);
        events.push(event);
    }
    events.sort_by(|event1, event2| event1.position.cmp_to_other(&event2.position, event1.direction));
    for event in events.iter() {
        let (position, direction) = (event.position, event.direction);
        if event.floor != Floor::Ice {
            break; //break in this loop means that this object and all that come before it stop movement
        }
        let object_position = board.get_next_position_for_move(position, direction);
        let object = board.get_object_type(object_position);
        match object {
            GameObject::Empty => {
                moved_writer.send(ExitedFloorEvent { floor: Floor::Ice, position, object: event.object, direction: event.direction });
            }
            GameObject::Box => {
                if positions.contains(&object_position)
                {
                    //found box is already moving
                    moved_writer.send(ExitedFloorEvent { floor: Floor::Ice, position, object: event.object, direction: event.direction });
                } else if board.get_floor_type(object_position) == Floor::Ice {
                    // if there are multiple stationary boxes ahead, either the last one moves
                    // (if it's on ice) or they remain stationary otherwise
                    let mut last_box_position = object_position;
                    let mut next_object_position =
                        board.get_next_position_for_move(last_box_position, direction);
                    let mut next_object = board.get_object_type(next_object_position);
                    while next_object == GameObject::Box
                        && board.get_floor_type(next_object_position) == Floor::Ice
                    {
                        last_box_position = next_object_position;
                        next_object_position =
                            board.get_next_position_for_move(next_object_position, direction);
                        next_object = board.get_object_type(next_object_position);
                    }
                    if next_object == GameObject::Empty {
                        moved_writer.send(ExitedFloorEvent { floor: Floor::Ice, position: last_box_position, object: event.object, direction: event.direction });
                    }
                    break;
                    //either way the entity that encountered a stationary entity in front of it must stop, and so do entities before it
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
}
