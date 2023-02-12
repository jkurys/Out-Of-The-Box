use bevy::prelude::*;

use crate::game::resources::Board;

use super::events::{EnteredFloorEvent, ExitedFloorEvent};

pub fn handle_move(
    mut reader: EventReader<ExitedFloorEvent>,
    mut board: ResMut<Board>,
    mut writer: EventWriter<EnteredFloorEvent>,
) {
    //updates the positions in MoveEvent without checking anything. We assume that the positions are already correct
    for event in reader.iter() {
        let next_position = event.position.next_position(event.direction);
        let floor = board.get_floor_type(next_position);
        board.move_object(event.position, event.direction);
        writer.send(EnteredFloorEvent {
            direction: event.direction,
            floor,
            object: event.object,
            position: next_position,
        });
    }
}
