use bevy::prelude::*;
use itertools::Itertools;

use super::events::{EnteredFloorEvent, TryMoveEvent};
use crate::{
    board::GameData,
    game::game_objects::{Block, Direction, Position},
};

pub fn handle_ice(
    mut writer: EventWriter<TryMoveEvent>,
    mut position_reader: EventReader<EnteredFloorEvent>,
    game_data: Res<GameData>,
) {
    let board = &game_data.board;
    let mut positions = Vec::new();
    for event in position_reader.read() {
        positions.push((event.position, event.direction));
    }
    let blocks: Vec<(Block, Direction, Position)> = positions
        .into_iter()
        .map(|(p, d)| (board.get_block(p), d, p))
        .unique()
        .collect();

    for (block, direction, position) in blocks.into_iter() {
        writer.write(TryMoveEvent {
            block,
            direction,
            position,
            is_weak: true,
            is_long: false,
        });
    }
}
