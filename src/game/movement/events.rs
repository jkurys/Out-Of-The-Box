use crate::game::game_objects::{Block, Direction, Floor, GameObject, Position};
use bevy::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Message)]
pub struct TryMoveMessage {
    pub block: Block,
    pub direction: Direction,
    pub is_weak: bool,
    pub position: Position,
    pub is_long: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Message)]
pub struct EnteredFloorMessage {
    pub floor: Floor,
    pub position: Position,
    pub object: GameObject,
    pub direction: Direction,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Message)]
pub struct TeleportMessage {
    pub position1: Position,
    pub position2: Position,
}
