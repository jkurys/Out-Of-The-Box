use bevy::{prelude::*, utils::HashSet};

use super::events::{EnteredFloorEvent, TryMoveEvent};
use crate::game::game_objects::{Block, Floor};

pub fn handle_ice(
    mut writer: EventWriter<TryMoveEvent>,
    mut position_reader: EventReader<EnteredFloorEvent>,
) {
    let mut events = Vec::new();
    let mut positions = Vec::new();
    for event in position_reader.iter() {
        positions.push(event.position);
        events.push(event);
    }
    events.sort_by(|event1, event2| {
        event1
            .position
            .cmp_to_other(&event2.position, event1.direction)
    });
    for event in events.iter() {
        let &&EnteredFloorEvent {
            position,
            direction,
            ..
        } = event;
        if event.floor != Floor::Ice {
            continue;
        }
        writer.send(TryMoveEvent {
            block: Block {
                positions: HashSet::from([position]),
            },
            direction,
            is_weak: true,
            insert_after: None,
        });
    }
}
