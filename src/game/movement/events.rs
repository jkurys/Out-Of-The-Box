use bevy::prelude::Event;

use crate::game::game_objects::{Direction, Floor, GameObject, Position};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Event)]
pub struct TryMoveEvent {
    pub position: Position,
    pub direction: Direction,
    pub is_weak: bool,
    pub insert_after: Option<(GameObject, Position)>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Event)]
pub struct EnteredFloorEvent {
    pub floor: Floor,
    pub position: Position,
    pub object: GameObject,
    pub direction: Direction,
}
