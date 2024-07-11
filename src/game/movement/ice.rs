use bevy::prelude::*;
use itertools::Itertools;

use super::events::{EnteredFloorEvent, TryMoveEvent};
use crate::{
    board::Board,
    game::game_objects::{Block, Direction},
};

pub fn handle_ice(
    mut writer: EventWriter<TryMoveEvent>,
    mut position_reader: EventReader<EnteredFloorEvent>,
    board: Res<Board>,
) {
    let mut positions = Vec::new();
    for event in position_reader.iter() {
        positions.push((event.position, event.direction));
    }
    let blocks: Vec<(Block, Direction)> = positions
        .into_iter()
        .map(|(p, d)| (board.get_block(p), d))
        .unique()
        .collect();

    for (block, direction) in blocks.into_iter() {
        writer.send(TryMoveEvent {
            block,
            direction,
            is_weak: true,
        });
    }
}
