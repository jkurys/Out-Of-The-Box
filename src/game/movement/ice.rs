use bevy::{prelude::*, utils::HashSet};

use super::resources::{MoveData, MoveObject, PushAttempt};
use crate::game::game_objects::{Block, Floor};

pub fn handle_ice(
    // mut writer: EventWriter<TryMoveEvent>,
    // mut position_reader: EventReader<EnteredFloorEvent>,
    mut move_data: ResMut<MoveData>
) {
    let mut positions = Vec::new();
    for mov in move_data.moves.iter() {
        positions.push(mov.position);
    }
    move_data.moves.sort_by(|mov1, mov2| {
        mov1
            .position
            .cmp_to_other(&mov2.position, mov1.direction)
    });
    for mov in move_data.moves.clone().iter() {
        let &MoveObject {
            position,
            direction,
            floor,
            ..
        } = mov;
        if floor != Floor::Ice {
            continue;
        }
        move_data.push_atempts.push(PushAttempt {
            block: Block {
                positions: HashSet::from([position]),
            },
            direction,
            is_weak: true,
            insert_after: None,
        });
    }
    move_data.moves.clear();
}
