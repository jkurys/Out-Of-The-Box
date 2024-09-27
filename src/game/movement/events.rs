use bevy::prelude::Event;

use crate::game::game_objects::{Block, Direction, Floor, GameObject, Position};

#[derive(Clone, PartialEq, Eq, Debug, Event)]
pub struct TryMoveEvent {
    pub block: Block,
    pub direction: Direction,
    pub is_weak: bool,
    pub position: Position,
    pub is_long: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Event)]
pub struct EnteredFloorEvent {
    pub floor: Floor,
    pub position: Position,
    pub object: GameObject,
    pub direction: Direction,
}
